//! Handler dispatch declarations and inherent methods — `GrpcHandlerRegistryDispatcher`.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_observ_metrics::MetricsProvider;

use super::grpc_handler_adapter::GrpcHandlerAdapter;

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
    use edge_domain::{Handler, HandlerError, HandlerRegistry};
    use std::sync::Arc;

    use crate::api::handler::grpc::grpc_handler_adapter::GrpcHandlerAdapter;
    use crate::api::port::grpc_inbound::GrpcInboundError;

    #[derive(Debug, PartialEq, Eq)]
    struct TestReq {
        value: u32,
    }
    #[derive(Debug, PartialEq, Eq)]
    struct TestResp {
        value: u32,
    }

    fn decode_test_req(bytes: &[u8]) -> Result<TestReq, GrpcInboundError> {
        if bytes.len() != 4 {
            return Err(GrpcInboundError::InvalidArgument(format!(
                "expected 4 bytes, got {}",
                bytes.len()
            )));
        }
        Ok(TestReq {
            value: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        })
    }
    fn encode_test_resp(resp: &TestResp) -> Vec<u8> {
        resp.value.to_be_bytes().to_vec()
    }

    struct DoublingHandler;
    #[async_trait::async_trait]
    impl Handler<TestReq, TestResp> for DoublingHandler {
        fn id(&self) -> &str {
            "/pkg.Service/Double"
        }
        fn pattern(&self) -> &str {
            "test"
        }
        async fn execute(&self, req: TestReq) -> Result<TestResp, HandlerError> {
            Ok(TestResp {
                value: req.value.wrapping_mul(2),
            })
        }
    }

    #[test]
    fn test_new_dispatcher_creates_empty_registry() {
        let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
        assert!(d.registry().is_empty());
    }

    /// @covers: registry
    #[test]
    fn test_registry_returns_shared_registry() {
        let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
        let r1 = d.registry().clone();
        let r2 = d.registry().clone();
        assert!(Arc::ptr_eq(&r1, &r2));
    }

    /// @covers: register
    #[test]
    fn test_register_adds_handler_to_registry() {
        let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
        d.register(GrpcHandlerAdapter::new(
            Arc::new(DoublingHandler),
            decode_test_req,
            encode_test_resp,
        ));
        assert!(!d.registry().is_empty());
    }

    /// @covers: with_metrics
    #[test]
    fn test_with_metrics_attaches_metrics_provider() {
        use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
            .with_metrics(Arc::clone(&provider));
        assert!(d.metrics.is_some());
    }
}
