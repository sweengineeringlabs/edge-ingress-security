//! HTTP handler registry dispatcher.

use std::sync::Arc;

use edge_domain::{Handler as _, HandlerRegistry};
use parking_lot::RwLock;
use swe_observ_metrics::MetricsProvider;

use crate::api::handler::error::HttpDispatcherError;
use crate::api::handler::types::http_handler_adapter::HttpHandlerAdapter;
use crate::api::vo::{HttpRequest, HttpResponse};

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
