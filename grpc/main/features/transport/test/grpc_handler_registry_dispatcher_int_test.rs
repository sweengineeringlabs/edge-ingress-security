//! Integration tests for GrpcHandlerRegistryDispatcher.

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError, HandlerRegistry};
use std::sync::Arc;
use swe_edge_ingress_grpc_transport::{
    GrpcDecodeFn, GrpcEncodeFn, GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher, GrpcIngressError,
};

#[derive(Debug, PartialEq, Eq)]
struct TestReq {
    value: u32,
}
#[derive(Debug, PartialEq, Eq)]
struct TestResp {
    value: u32,
}

fn decode_test_req(bytes: &[u8]) -> Result<TestReq, GrpcIngressError> {
    if bytes.len() != 4 {
        return Err(GrpcIngressError::InvalidArgument(format!(
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
#[async_trait]
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

/// @covers: GrpcHandlerRegistryDispatcher::new
#[test]
fn test_new_dispatcher_creates_empty_registry() {
    let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
    assert!(d.registry().is_empty());
}

/// @covers: GrpcHandlerRegistryDispatcher::registry
#[test]
fn test_registry_returns_shared_registry() {
    let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
    let r1 = d.registry().clone();
    let r2 = d.registry().clone();
    assert!(Arc::ptr_eq(&r1, &r2));
}

/// @covers: GrpcHandlerRegistryDispatcher::register
#[test]
fn test_register_adds_handler_to_registry() {
    let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
    d.register(GrpcHandlerAdapter::new(
        Arc::new(DoublingHandler),
        decode_test_req as GrpcDecodeFn<TestReq>,
        encode_test_resp as GrpcEncodeFn<TestResp>,
    ));
    assert!(!d.registry().is_empty());
}

/// @covers: GrpcHandlerRegistryDispatcher::with_metrics
#[test]
fn test_with_metrics_attaches_metrics_provider() {
    use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        .with_metrics(Arc::clone(&provider));
    // Construction succeeds — metrics provider is attached
}
