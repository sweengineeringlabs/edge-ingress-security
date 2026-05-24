//! HTTP handler registry dispatcher.

use std::sync::Arc;

use edge_domain::{Handler as _, HandlerRegistry};
use parking_lot::RwLock;
use swe_observ_metrics::MetricsProvider;

use crate::api::handler::http::http_dispatcher_error::HttpDispatcherError;
use crate::api::handler::http::http_handler_adapter::HttpHandlerAdapter;
use crate::api::value_object::{HttpRequest, HttpResponse};

/// Dispatcher that routes inbound HTTP requests through a
/// [`HandlerRegistry`] keyed by handler id, using `matchit` path-pattern
/// matching.
pub struct HttpHandlerRegistryDispatcher {
    pub(crate) registry: Arc<HandlerRegistry<HttpRequest, HttpResponse>>,
    pub(crate) router: RwLock<matchit::Router<String>>,
    pub(crate) metrics: Option<Arc<dyn MetricsProvider>>,
}

impl HttpHandlerRegistryDispatcher {
    /// Construct a dispatcher backed by `registry`.
    pub fn new(registry: Arc<HandlerRegistry<HttpRequest, HttpResponse>>) -> Self {
        Self {
            registry,
            router: RwLock::new(matchit::Router::new()),
            metrics: None,
        }
    }

    /// Attach a metrics provider; per-handler counters and latency histograms
    /// are recorded automatically on every dispatch.
    pub fn with_metrics(mut self, provider: Arc<dyn MetricsProvider>) -> Self {
        self.metrics = Some(provider);
        self
    }

    /// Register a typed adapter.
    pub fn register<Req, Resp>(
        &self,
        adapter: HttpHandlerAdapter<Req, Resp>,
    ) -> Result<(), HttpDispatcherError>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        let id = adapter.id().to_string();
        let pattern = adapter.pattern().to_string();
        self.registry.register(Arc::new(adapter));
        self.router
            .write()
            .insert(pattern.clone(), id)
            .map_err(|e| HttpDispatcherError::RegistrationFailed {
                pattern,
                reason: e.to_string(),
            })
    }

    /// Borrow the inner registry.
    pub fn registry(&self) -> &Arc<HandlerRegistry<HttpRequest, HttpResponse>> {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain::HandlerRegistry;
    use std::sync::Arc;

    fn fresh_dispatcher() -> HttpHandlerRegistryDispatcher {
        HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
    }

    /// @covers: registry
    #[test]
    fn test_new_dispatcher_creates_empty_registry() {
        assert!(fresh_dispatcher().registry().is_empty());
    }

    /// @covers: registry
    #[test]
    fn test_registry_returns_shared_registry() {
        let d = fresh_dispatcher();
        let r1 = d.registry().clone();
        let r2 = d.registry().clone();
        assert!(Arc::ptr_eq(&r1, &r2));
    }

    /// @covers: with_metrics
    #[test]
    fn test_with_metrics_attaches_metrics_provider() {
        use swe_observ_metrics::create_local_metrics_backend;
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let d = fresh_dispatcher().with_metrics(Arc::clone(&provider));
        assert!(d.metrics.is_some());
    }

    /// @covers: register
    #[test]
    fn test_register_adds_handler_to_registry() {
        use crate::api::handler::http::http_handler_adapter::HttpHandlerAdapter;
        use crate::api::port::http_ingress_error::HttpIngressError;
        use crate::api::value_object::{HttpRequest, HttpResponse};
        use edge_domain::{Handler, HandlerError};

        struct PingHandler;
        #[async_trait::async_trait]
        impl Handler<HttpRequest, HttpResponse> for PingHandler {
            fn id(&self) -> &str {
                "ping"
            }
            fn pattern(&self) -> &str {
                "/ping"
            }
            async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
                Ok(HttpResponse::new(200, vec![]))
            }
        }

        fn dec(req: &HttpRequest) -> Result<HttpRequest, HttpIngressError> {
            Ok(req.clone())
        }
        fn enc(r: HttpResponse) -> HttpResponse {
            r
        }

        let d = fresh_dispatcher();
        d.register(HttpHandlerAdapter::new(Arc::new(PingHandler), dec, enc))
            .expect("register ok");
        assert_eq!(d.registry().len(), 1);
    }
}
