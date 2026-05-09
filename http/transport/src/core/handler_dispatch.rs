//! Registry-backed [`HttpInbound`] dispatcher implementation.

use std::sync::Arc;

use edge_domain::{Handler, HandlerError, HandlerRegistry};
use futures::future::BoxFuture;

use crate::api::handler_adapter::HttpHandlerAdapter;
use crate::api::handler_dispatch::{HttpDispatcherError, HttpHandlerRegistryDispatcher};
use crate::api::port::http_inbound::{
    HttpHealthCheck, HttpInbound, HttpInboundError, HttpInboundResult,
};
use crate::api::value_object::{HttpRequest, HttpResponse};


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

    struct PingHandler;
    #[async_trait]
    impl Handler<HttpRequest, HttpResponse> for PingHandler {
        fn id(&self) -> &str { "ping" }
        fn pattern(&self) -> &str { "/ping" }
        async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
            Ok(HttpResponse { status: 200, headers: Default::default(), body: Default::default() })
        }
        async fn health_check(&self) -> bool { true }
        fn as_any(&self) -> &dyn Any { self }
    }

    fn fresh_dispatcher() -> HttpHandlerRegistryDispatcher {
        HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
    }

    /// @covers: new — starts empty.
    #[test]
    fn test_new_dispatcher_starts_empty() {
        assert!(fresh_dispatcher().registry().is_empty());
    }

    /// @covers: register — adds adapter.
    #[test]
    fn test_register_adds_handler() {
        fn decode_req(req: &HttpRequest) -> Result<HttpRequest, HttpInboundError> { Ok(req.clone()) }
        fn encode_resp(r: HttpResponse) -> HttpResponse { r }
        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(PingHandler), decode_req, encode_resp))
            .expect("register ok");
        assert_eq!(d.registry().len(), 1);
    }

    /// @covers: map_handler_error — Unsupported maps to NotFound.
    #[test]
    fn test_map_handler_error_unsupported_maps_to_not_found() {
        assert!(matches!(map_handler_error(HandlerError::Unsupported("x".into())), HttpInboundError::NotFound(_)));
    }
}
