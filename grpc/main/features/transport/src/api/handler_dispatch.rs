//! Handler dispatch declarations and inherent methods — `GrpcHandlerRegistryDispatcher`.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_observ_metrics::MetricsProvider;

use crate::api::handler_adapter::GrpcHandlerAdapter;

/// Dispatcher that routes inbound gRPC calls through a byte-oriented
/// [`HandlerRegistry`] keyed by the gRPC method path.
pub struct GrpcHandlerRegistryDispatcher {
    pub(crate) registry: Arc<HandlerRegistry<Vec<u8>, Vec<u8>>>,
    pub(crate) metrics: Option<Arc<dyn MetricsProvider>>,
}

impl GrpcHandlerRegistryDispatcher {
    /// Construct a dispatcher backed by `registry`.
    pub fn new(registry: Arc<HandlerRegistry<Vec<u8>, Vec<u8>>>) -> Self {
        Self {
            registry,
            metrics: None,
        }
    }

    /// Attach a metrics provider; per-handler counters and latency histograms
    /// are recorded automatically on every dispatch.
    pub fn with_metrics(mut self, provider: Arc<dyn MetricsProvider>) -> Self {
        self.metrics = Some(provider);
        self
    }

    /// Register a typed adapter under its `id()`.
    pub fn register<Req, Resp>(&self, adapter: GrpcHandlerAdapter<Req, Resp>)
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        self.registry.register(Arc::new(adapter));
    }

    /// Borrow the inner registry.
    pub fn registry(&self) -> &Arc<HandlerRegistry<Vec<u8>, Vec<u8>>> {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::GrpcHandlerRegistryDispatcher;
    use edge_domain::HandlerRegistry;
    use std::sync::Arc;

    /// @covers: new — creates empty dispatcher.
    #[test]
    fn test_new_dispatcher_creates_empty_registry() {
        let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
        assert!(d.registry().is_empty());
    }

    /// @covers: registry — returns same Arc.
    #[test]
    fn test_registry_returns_shared_registry() {
        let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
        let r1 = d.registry().clone();
        let r2 = d.registry().clone();
        assert!(Arc::ptr_eq(&r1, &r2));
    }
}
