//! Registry-backed [`HttpInbound`] dispatcher with pattern routing.
//!
//! Wraps an `Arc<HandlerRegistry<HttpRequest, HttpResponse>>` from `edge-domain`
//! and implements [`HttpInbound`] by:
//!
//! 1. extracting the path from the inbound URL,
//! 2. matching it against registered `matchit` patterns (sourced from
//!    `Handler::pattern` at registration time),
//! 3. looking up the matching `Handler<HttpRequest, HttpResponse>` in the
//!    registry by its id,
//! 4. forwarding the full [`HttpRequest`] to `Handler::execute`.
//!
//! Path-not-found is reported as [`HttpInboundError::NotFound`].

use std::sync::Arc;

use edge_domain::{Handler, HandlerError, HandlerRegistry};
use futures::future::BoxFuture;
use parking_lot::RwLock;

use crate::api::handler_adapter::HttpHandlerAdapter;
use crate::api::port::http_inbound::{
    HttpHealthCheck, HttpInbound, HttpInboundError, HttpInboundResult,
};
use crate::api::value_object::{HttpRequest, HttpResponse};

/// Error returned when a handler registration fails.
#[derive(Debug, thiserror::Error)]
pub enum HttpDispatcherError {
    /// The route pattern is syntactically invalid or conflicts with an
    /// already-registered pattern.
    #[error("failed to register pattern `{pattern}`: {reason}")]
    RegistrationFailed {
        /// The pattern that failed to register.
        pattern: String,
        /// The reason for the failure.
        reason: String,
    },
}

/// Dispatcher that routes inbound HTTP requests through a
/// [`HandlerRegistry`] keyed by handler id, using `matchit` path-pattern
/// matching against each handler's [`Handler::pattern`].
///
/// Use this together with [`HttpHandlerAdapter`] to register typed
/// handlers and let the server dispatch the right one for each inbound
/// request path.
pub struct HttpHandlerRegistryDispatcher {
    registry: Arc<HandlerRegistry<HttpRequest, HttpResponse>>,
    router:   RwLock<matchit::Router<String>>,
}

impl HttpHandlerRegistryDispatcher {
    /// Construct a dispatcher backed by `registry`.
    pub fn new(registry: Arc<HandlerRegistry<HttpRequest, HttpResponse>>) -> Self {
        Self { registry, router: RwLock::new(matchit::Router::new()) }
    }

    /// Register a typed adapter.
    ///
    /// The adapter's `pattern()` (e.g. `"/users/{id}"`) is inserted into the
    /// path router; `id()` is the registry key used at dispatch time.
    /// Patterns follow matchit 0.8 syntax — parameters are wrapped in braces
    /// (`{param}`), wildcards use `{*param}`.
    /// Returns an error when the pattern is syntactically invalid or
    /// conflicts with an already-registered pattern.
    pub fn register<Req, Resp>(
        &self,
        adapter: HttpHandlerAdapter<Req, Resp>,
    ) -> Result<(), HttpDispatcherError>
    where
        Req:  Send + 'static,
        Resp: Send + 'static,
    {
        let id      = adapter.id().to_string();
        let pattern = adapter.pattern().to_string();
        self.registry.register(Arc::new(adapter));
        self.router.write().insert(pattern.clone(), id).map_err(|e| {
            HttpDispatcherError::RegistrationFailed { pattern, reason: e.to_string() }
        })
    }

    /// Borrow the inner registry — callers can list ids, deregister,
    /// or share the registry with administrative tooling.
    pub fn registry(&self) -> &Arc<HandlerRegistry<HttpRequest, HttpResponse>> {
        &self.registry
    }
}

impl HttpInbound for HttpHandlerRegistryDispatcher {
    fn handle(&self, request: HttpRequest) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        Box::pin(async move {
            let path = path_from_url(&request.url);
            let id = {
                let router = self.router.read();
                match router.at(&path) {
                    Ok(m)  => m.value.clone(),
                    Err(_) => {
                        return Err(HttpInboundError::NotFound(format!(
                            "no handler registered for {path}"
                        )));
                    }
                }
            };
            let handler = match self.registry.get(&id) {
                Some(h) => h,
                None    => {
                    return Err(HttpInboundError::Internal(format!(
                        "route matched `{id}` but handler was not found in registry"
                    )));
                }
            };
            match handler.execute(request).await {
                Ok(resp) => Ok(resp),
                Err(e)   => Err(map_handler_error(e)),
            }
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        let registry = self.registry.clone();
        Box::pin(async move {
            let ids = registry.list_ids();
            for id in ids {
                if let Some(h) = registry.get(&id) {
                    if !h.health_check().await {
                        return Ok(HttpHealthCheck::unhealthy(format!(
                            "handler {id} reported unhealthy"
                        )));
                    }
                }
            }
            Ok(HttpHealthCheck::healthy())
        })
    }
}

/// Extract the path component from a URL string, stripping query and fragment.
///
/// Handles both absolute URLs (`https://host/path?q=1`) and
/// path-only strings (`/path?q=1`).
fn path_from_url(url: &str) -> String {
    url.parse::<http::Uri>()
        .map(|u| u.path().to_string())
        .unwrap_or_else(|_| {
            url.split('?').next()
               .and_then(|s| s.split('#').next())
               .unwrap_or("/")
               .to_string()
        })
}

fn map_handler_error(err: HandlerError) -> HttpInboundError {
    match err {
        HandlerError::Unsupported(m)        => HttpInboundError::NotFound(m),
        HandlerError::InvalidRequest(m)     => HttpInboundError::InvalidInput(m),
        HandlerError::ExecutionFailed(m)    => HttpInboundError::Internal(m),
        HandlerError::Unhealthy             => HttpInboundError::Unavailable("handler unhealthy".into()),
        HandlerError::FailedPrecondition(m) => HttpInboundError::InvalidInput(m),
        HandlerError::Other(m)              => HttpInboundError::Internal(m),
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::Arc;

    use async_trait::async_trait;
    use edge_domain::{Handler, HandlerError, HandlerRegistry};

    use super::*;
    use crate::api::handler_adapter::HttpHandlerAdapter;
    use crate::api::port::http_inbound::HttpInbound;
    use crate::api::value_object::HttpRequest;

    // ── Stub handlers ─────────────────────────────────────────────────────────

    struct PingHandler;

    #[async_trait]
    impl Handler<HttpRequest, HttpResponse> for PingHandler {
        fn id(&self) -> &str { "ping" }
        fn pattern(&self) -> &str { "/ping" }
        async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
            Ok(HttpResponse::new(200, b"pong".to_vec()))
        }
        async fn health_check(&self) -> bool { true }
        fn as_any(&self) -> &dyn Any { self }
    }

    struct SickHandler;

    #[async_trait]
    impl Handler<HttpRequest, HttpResponse> for SickHandler {
        fn id(&self) -> &str { "sick" }
        fn pattern(&self) -> &str { "/sick" }
        async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
            Err(HandlerError::Unhealthy)
        }
        async fn health_check(&self) -> bool { false }
        fn as_any(&self) -> &dyn Any { self }
    }

    fn identity_decode(req: &HttpRequest) -> Result<HttpRequest, HttpInboundError> { Ok(req.clone()) }
    fn identity_encode(resp: HttpResponse) -> HttpResponse { resp }

    fn fresh_dispatcher() -> HttpHandlerRegistryDispatcher {
        HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// @covers: new — empty registry has no handlers.
    #[tokio::test]
    async fn test_new_dispatcher_starts_with_no_registered_handlers() {
        let d = fresh_dispatcher();
        assert!(d.registry().is_empty());
    }

    /// @covers: register — adapter inserted under handler id, pattern wired.
    #[tokio::test]
    async fn test_register_inserts_adapter_under_handler_id() {
        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(PingHandler), identity_decode, identity_encode))
            .expect("register ok");
        assert_eq!(d.registry().len(), 1);
        assert!(d.registry().get("ping").is_some());
    }

    /// @covers: handle — registered handler runs and returns correct response.
    #[tokio::test]
    async fn test_handle_runs_handler_and_returns_response() {
        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(PingHandler), identity_decode, identity_encode))
            .expect("register ok");
        let resp = d.handle(HttpRequest::get("/ping")).await.expect("dispatch ok");
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body,   b"pong");
    }

    /// @covers: handle — unregistered path returns NotFound.
    #[tokio::test]
    async fn test_handle_returns_not_found_when_no_handler_matches_path() {
        let d   = fresh_dispatcher();
        let err = d.handle(HttpRequest::get("/missing")).await.expect_err("must error");
        assert!(matches!(err, HttpInboundError::NotFound(_)));
    }

    /// @covers: handle — matchit parameterized pattern is matched correctly.
    #[tokio::test]
    async fn test_handle_matches_parameterized_path_pattern() {
        struct UserHandler;
        #[async_trait]
        impl Handler<HttpRequest, HttpResponse> for UserHandler {
            fn id(&self) -> &str { "get-user" }
            fn pattern(&self) -> &str { "/users/{id}" }
            async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
                Ok(HttpResponse::new(200, vec![]))
            }
            async fn health_check(&self) -> bool { true }
            fn as_any(&self) -> &dyn Any { self }
        }
        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(UserHandler), identity_decode, identity_encode))
            .expect("register ok");
        let resp = d.handle(HttpRequest::get("/users/42")).await.expect("dispatch ok");
        assert_eq!(resp.status, 200);
    }

    /// @covers: register — conflicting patterns return RegistrationFailed.
    #[tokio::test]
    async fn test_register_returns_error_for_conflicting_patterns() {
        struct HandlerA;
        struct HandlerB;
        #[async_trait]
        impl Handler<HttpRequest, HttpResponse> for HandlerA {
            fn id(&self) -> &str { "a" }
            fn pattern(&self) -> &str { "/foo/{id}" }
            async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
                Ok(HttpResponse::new(200, vec![]))
            }
            async fn health_check(&self) -> bool { true }
            fn as_any(&self) -> &dyn Any { self }
        }
        #[async_trait]
        impl Handler<HttpRequest, HttpResponse> for HandlerB {
            fn id(&self) -> &str { "b" }
            fn pattern(&self) -> &str { "/foo/{name}" }
            async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
                Ok(HttpResponse::new(200, vec![]))
            }
            async fn health_check(&self) -> bool { true }
            fn as_any(&self) -> &dyn Any { self }
        }
        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(HandlerA), identity_decode, identity_encode))
            .expect("first register ok");
        let err = d.register(HttpHandlerAdapter::new(Arc::new(HandlerB), identity_decode, identity_encode))
            .expect_err("conflict must error");
        assert!(matches!(err, HttpDispatcherError::RegistrationFailed { .. }));
    }

    /// @covers: health_check — empty registry is healthy.
    #[tokio::test]
    async fn test_health_check_returns_healthy_for_empty_registry() {
        let h = fresh_dispatcher().health_check().await.expect("health ok");
        assert!(h.healthy);
    }

    /// @covers: health_check — unhealthy handler taints aggregate.
    #[tokio::test]
    async fn test_health_check_returns_unhealthy_when_any_handler_is_unhealthy() {
        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(PingHandler), identity_decode, identity_encode))
            .expect("register ok");
        d.register(HttpHandlerAdapter::new(Arc::new(SickHandler), identity_decode, identity_encode))
            .expect("register ok");
        let h = d.health_check().await.expect("health ok");
        assert!(!h.healthy);
        assert!(h.message.unwrap().contains("sick"));
    }

    /// @covers: map_handler_error — Unsupported -> NotFound.
    #[test]
    fn test_map_handler_error_unsupported_maps_to_not_found() {
        assert!(matches!(
            map_handler_error(HandlerError::Unsupported("x".into())),
            HttpInboundError::NotFound(_)
        ));
    }

    /// @covers: map_handler_error — InvalidRequest -> InvalidInput.
    #[test]
    fn test_map_handler_error_invalid_request_maps_to_invalid_input() {
        assert!(matches!(
            map_handler_error(HandlerError::InvalidRequest("x".into())),
            HttpInboundError::InvalidInput(_)
        ));
    }

    /// @covers: map_handler_error — ExecutionFailed -> Internal.
    #[test]
    fn test_map_handler_error_execution_failed_maps_to_internal() {
        assert!(matches!(
            map_handler_error(HandlerError::ExecutionFailed("x".into())),
            HttpInboundError::Internal(_)
        ));
    }

    /// @covers: map_handler_error — Unhealthy -> Unavailable.
    #[test]
    fn test_map_handler_error_unhealthy_maps_to_unavailable() {
        assert!(matches!(
            map_handler_error(HandlerError::Unhealthy),
            HttpInboundError::Unavailable(_)
        ));
    }

    /// @covers: map_handler_error — Other -> Internal.
    #[test]
    fn test_map_handler_error_other_maps_to_internal() {
        assert!(matches!(
            map_handler_error(HandlerError::Other("x".into())),
            HttpInboundError::Internal(_)
        ));
    }

    /// @covers: path_from_url — extracts path from absolute URL, strips query.
    #[test]
    fn test_path_from_url_extracts_path_from_absolute_url() {
        assert_eq!(path_from_url("https://example.com/api/users?page=1"), "/api/users");
    }

    /// @covers: path_from_url — path-only string returned unchanged.
    #[test]
    fn test_path_from_url_returns_path_only_string_unchanged() {
        assert_eq!(path_from_url("/api/users"), "/api/users");
    }

    /// @covers: path_from_url — query string stripped from path-only string.
    #[test]
    fn test_path_from_url_strips_query_string_from_path_only_string() {
        assert_eq!(path_from_url("/api/users?foo=bar"), "/api/users");
    }
}
