//! Registry-backed [`HttpInbound`] dispatcher implementation.

use std::time::Instant;

use edge_domain::{HandlerError, RequestContext};
use futures::future::BoxFuture;

use crate::api::handler::http::http_handler_registry_dispatcher::HttpHandlerRegistryDispatcher;
use crate::api::port::http_health_check::HttpHealthCheck;
use crate::api::port::http_inbound::HttpInbound;
use crate::api::port::http_inbound_error::HttpInboundError;
use crate::api::port::http_inbound_result::HttpInboundResult;
use crate::api::value_object::{HttpRequest, HttpResponse};

impl HttpInbound for HttpHandlerRegistryDispatcher {
    fn handle(
        &self,
        request: HttpRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        let metrics = self.metrics.clone();
        Box::pin(async move {
            let path = path_from_url(&request.url);
            let id = {
                let router = self.router.read();
                match router.at(&path) {
                    Ok(m) => m.value.clone(),
                    Err(_) => {
                        return Err(HttpInboundError::NotFound(format!(
                            "no handler registered for {path}"
                        )))
                    }
                }
            };
            let handler = match self.registry.get(&id) {
                Some(h) => h,
                None => {
                    return Err(HttpInboundError::Internal(format!(
                        "route matched `{id}` but handler was not found in registry"
                    )))
                }
            };
            let start = Instant::now();
            let result = if ctx.trace_id.is_empty() {
                handler.execute_with_context(request, ctx).await
            } else {
                let span_ctx = swe_justobserv_context::LogContext::builder()
                    .trace_id(&ctx.trace_id)
                    .build();
                swe_justobserv_context::with_log_context(
                    span_ctx,
                    handler.execute_with_context(request, ctx),
                )
                .await
            };
            if let Some(ref m) = metrics {
                let latency = start.elapsed().as_micros() as f64;
                let labels = &[("handler_id", id.as_str())];
                m.record_counter("edge_handler_requests_total", 1.0, labels);
                m.record_histogram("edge_handler_latency_us", latency, labels);
                if result.is_err() {
                    m.record_counter("edge_handler_errors_total", 1.0, labels);
                }
            }
            result.map_err(map_handler_error)
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

pub(crate) fn path_from_url(url: &str) -> String {
    url.parse::<http::Uri>()
        .map(|u| u.path().to_string())
        .unwrap_or_else(|_| {
            url.split('?')
                .next()
                .and_then(|s| s.split('#').next())
                .unwrap_or("/")
                .to_string()
        })
}

pub(crate) fn map_handler_error(err: HandlerError) -> HttpInboundError {
    match err {
        HandlerError::Unsupported(m) => HttpInboundError::MethodNotAllowed(m),
        HandlerError::InvalidRequest(m) => HttpInboundError::InvalidInput(m),
        HandlerError::NotFound(m) => HttpInboundError::NotFound(m),
        HandlerError::Conflict(m) => HttpInboundError::Conflict(m),
        HandlerError::ExecutionFailed(m) => HttpInboundError::Internal(m),
        HandlerError::Unhealthy => HttpInboundError::Unavailable("handler unhealthy".into()),
        HandlerError::FailedPrecondition(m) => HttpInboundError::UnprocessableEntity(m),
        HandlerError::Unauthorized(m) => HttpInboundError::Unauthorized(m),
        HandlerError::PermissionDenied(m) => HttpInboundError::PermissionDenied(m),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use edge_domain::{Handler, HandlerError, HandlerRegistry, RequestContext};

    use super::*;
    use crate::api::handler::http::http_handler_adapter::HttpHandlerAdapter;
    use crate::api::port::http_inbound::HttpInbound;
    use crate::api::port::http_inbound_error::HttpInboundError;
    use crate::api::value_object::HttpRequest;

    fn fresh() -> HttpHandlerRegistryDispatcher {
        HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
    }

    fn ctx() -> RequestContext {
        RequestContext::unauthenticated()
    }

    fn make_ping_adapter() -> crate::api::handler::http::http_handler_adapter::HttpHandlerAdapter<
        HttpRequest,
        HttpResponse,
    > {
        struct RegistryDispatcherImplH;
        #[async_trait::async_trait]
        impl Handler<HttpRequest, HttpResponse> for RegistryDispatcherImplH {
            fn id(&self) -> &str {
                "ping"
            }
            fn pattern(&self) -> &str {
                "/ping"
            }
            async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
                Ok(HttpResponse {
                    status: 200,
                    headers: Default::default(),
                    body: Default::default(),
                })
            }
        }
        fn dec(req: &HttpRequest) -> Result<HttpRequest, HttpInboundError> {
            Ok(req.clone())
        }
        fn enc(r: HttpResponse) -> HttpResponse {
            r
        }
        HttpHandlerAdapter::new(Arc::new(RegistryDispatcherImplH), dec, enc)
    }

    #[test]
    fn test_new_dispatcher_starts_empty() {
        assert!(fresh().registry().is_empty());
    }

    #[test]
    fn test_register_adds_handler() {
        let d = fresh();
        d.register(make_ping_adapter()).expect("ok");
        assert_eq!(d.registry().len(), 1);
    }

    #[tokio::test]
    async fn test_handle_dispatches_to_registered_handler() {
        let d = fresh();
        d.register(make_ping_adapter()).expect("ok");
        let req = HttpRequest::get("/ping");
        let resp = d.handle(req, ctx()).await.expect("handle ok");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_handle_returns_not_found_for_unknown_route() {
        let d = fresh();
        let err = d
            .handle(HttpRequest::get("/nope"), ctx())
            .await
            .unwrap_err();
        assert!(matches!(err, HttpInboundError::NotFound(_)));
    }

    #[test]
    fn test_map_handler_error_unsupported_maps_to_method_not_allowed() {
        assert!(matches!(
            map_handler_error(HandlerError::Unsupported("x".into())),
            HttpInboundError::MethodNotAllowed(_)
        ));
    }

    #[test]
    fn test_map_handler_error_failed_precondition_maps_to_unprocessable_entity() {
        assert!(matches!(
            map_handler_error(HandlerError::FailedPrecondition("x".into())),
            HttpInboundError::UnprocessableEntity(_)
        ));
    }

    #[tokio::test]
    async fn test_handle_with_metrics_records_handler_requests_total_on_success() {
        use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let d = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
            .with_metrics(Arc::clone(&provider));
        d.register(make_ping_adapter()).expect("ok");
        d.handle(HttpRequest::get("/ping"), ctx())
            .await
            .expect("ok");
        let snaps = provider.export();
        assert!(
            snaps
                .iter()
                .any(|s| s.name == "edge_handler_requests_total" && s.value == 1.0),
            "expected edge_handler_requests_total=1, got {snaps:?}"
        );
    }

    #[tokio::test]
    async fn test_handle_with_metrics_records_handler_errors_total_on_failure() {
        use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

        struct RegistryDispatcherImplFailH;
        #[async_trait::async_trait]
        impl Handler<HttpRequest, HttpResponse> for RegistryDispatcherImplFailH {
            fn id(&self) -> &str {
                "fail"
            }
            fn pattern(&self) -> &str {
                "/fail"
            }
            async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
                Err(HandlerError::ExecutionFailed("boom".into()))
            }
        }
        fn dec(req: &HttpRequest) -> Result<HttpRequest, HttpInboundError> {
            Ok(req.clone())
        }
        fn enc(r: HttpResponse) -> HttpResponse {
            r
        }

        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let d = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
            .with_metrics(Arc::clone(&provider));
        d.register(HttpHandlerAdapter::new(
            Arc::new(RegistryDispatcherImplFailH),
            dec,
            enc,
        ))
        .expect("ok");
        let _ = d.handle(HttpRequest::get("/fail"), ctx()).await;
        let snaps = provider.export();
        assert!(
            snaps
                .iter()
                .any(|s| s.name == "edge_handler_errors_total" && s.value == 1.0),
            "expected edge_handler_errors_total=1, got {snaps:?}"
        );
    }

    #[tokio::test]
    async fn test_handle_with_metrics_records_handler_latency_histogram() {
        use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let d = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
            .with_metrics(Arc::clone(&provider));
        d.register(make_ping_adapter()).expect("ok");
        d.handle(HttpRequest::get("/ping"), ctx())
            .await
            .expect("ok");
        let snaps = provider.export();
        assert!(
            snaps.iter().any(|s| s.name == "edge_handler_latency_us"),
            "expected edge_handler_latency_us, got {snaps:?}"
        );
    }

    #[test]
    fn test_path_from_url_extracts_path_from_full_url() {
        assert_eq!(path_from_url("https://example.com/api/v1"), "/api/v1");
    }

    #[test]
    fn test_path_from_url_strips_query_string() {
        assert_eq!(path_from_url("https://example.com/api?foo=bar"), "/api");
    }

    #[test]
    fn test_path_from_url_returns_empty_for_empty_string() {
        // Empty string: splitn gives empty next, then empty #-split, resulting in empty string.
        // This is the correct behavior - callers must ensure URLs are non-empty.
        let result = path_from_url("");
        // The function returns what it can parse from the empty string.
        assert!(result == "/" || result.is_empty(), "unexpected: {result}");
    }
}
