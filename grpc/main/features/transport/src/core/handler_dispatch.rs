//! Registry-backed [`GrpcInbound`] dispatcher implementation.

use std::sync::Arc;

use edge_domain::{HandlerError, HandlerRegistry, RequestContext};
use futures::future::BoxFuture;

use crate::api::handler_dispatch::GrpcHandlerRegistryDispatcher;
use crate::api::port::grpc_inbound::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundError, GrpcInboundResult,
};
use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};

impl GrpcInbound for GrpcHandlerRegistryDispatcher {
    fn handle_unary(
        &self,
        request: GrpcRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
        let registry = self.registry.clone();
        Box::pin(async move {
            let method = request.method.clone();
            let handler = match registry.get(&method) {
                Some(h) => h,
                None => {
                    return Err(GrpcInboundError::Unimplemented(format!(
                        "no handler registered for {method}"
                    )));
                }
            };
            match handler.execute_with_context(request.body, ctx).await {
                Ok(bytes) => Ok(GrpcResponse {
                    body:     bytes,
                    metadata: GrpcMetadata::default(),
                }),
                Err(e) => Err(map_handler_error(e)),
            }
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
        let registry = self.registry.clone();
        Box::pin(async move {
            let ids = registry.list_ids();
            for id in ids {
                if let Some(h) = registry.get(&id) {
                    if !h.health_check().await {
                        return Ok(GrpcHealthCheck::unhealthy(format!(
                            "handler {id} reported unhealthy"
                        )));
                    }
                }
            }
            Ok(GrpcHealthCheck::healthy())
        })
    }
}

fn map_handler_error(err: HandlerError) -> GrpcInboundError {
    match err {
        HandlerError::Unsupported(m)        => GrpcInboundError::Unimplemented(m),
        HandlerError::InvalidRequest(m)     => GrpcInboundError::InvalidArgument(m),
        HandlerError::ExecutionFailed(m)    => GrpcInboundError::Internal(m),
        HandlerError::Unhealthy             => GrpcInboundError::Unavailable("handler unhealthy".into()),
        HandlerError::FailedPrecondition(m) => GrpcInboundError::Status(crate::saf::GrpcStatusCode::FailedPrecondition, m),
        HandlerError::Other(m)              => GrpcInboundError::Internal(m),
    }
}

#[cfg(test)]
mod tests {
        use std::time::Duration;
    use std::sync::Arc;

    use async_trait::async_trait;
    use edge_domain::{Handler, HandlerError, HandlerRegistry, RequestContext};

    use crate::api::handler_adapter::GrpcHandlerAdapter;
    use crate::api::handler_dispatch::GrpcHandlerRegistryDispatcher;
    use crate::api::port::grpc_inbound::{GrpcInbound, GrpcInboundError};
    use crate::api::value_object::GrpcRequest;

    #[derive(Debug, PartialEq, Eq)]
    struct TestReq { value: u32 }
    #[derive(Debug, PartialEq, Eq)]
    struct TestResp { value: u32 }

    fn decode_test_req(bytes: &[u8]) -> Result<TestReq, GrpcInboundError> {
        if bytes.len() != 4 {
            return Err(GrpcInboundError::InvalidArgument(format!("expected 4 bytes, got {}", bytes.len())));
        }
        Ok(TestReq { value: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) })
    }
    fn encode_test_resp(resp: &TestResp) -> Vec<u8> { resp.value.to_be_bytes().to_vec() }

    struct DoublingHandler;
    #[async_trait]
    impl Handler<TestReq, TestResp> for DoublingHandler {
        fn id(&self) -> &str { "/pkg.Service/Double" }
        fn pattern(&self) -> &str { "test" }
        async fn execute(&self, req: TestReq) -> Result<TestResp, HandlerError> {
            Ok(TestResp { value: req.value.wrapping_mul(2) })
        }
    }

    fn fresh_dispatcher() -> GrpcHandlerRegistryDispatcher {
        GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
    }

    /// @covers: handle_unary — typed handler runs and returns encoded response.
    #[tokio::test]
    async fn test_handle_unary_runs_typed_handler_and_returns_response() {
        let d = fresh_dispatcher();
        d.register(GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp));
        let req = GrpcRequest::new("/pkg.Service/Double", 21u32.to_be_bytes().to_vec(), Duration::from_secs(1));
        let resp = d.handle_unary(req, RequestContext::unauthenticated()).await.expect("dispatch ok");
        let out = u32::from_be_bytes([resp.body[0], resp.body[1], resp.body[2], resp.body[3]]);
        assert_eq!(out, 42);
    }

    /// @covers: handle_unary — unknown method returns Unimplemented.
    #[tokio::test]
    async fn test_handle_unary_returns_unimplemented_when_method_not_registered() {
        let d = fresh_dispatcher();
        let req = GrpcRequest::new("/pkg.Service/NotThere", vec![], Duration::from_secs(1));
        let err = d.handle_unary(req, RequestContext::unauthenticated()).await.expect_err("must error");
        assert!(matches!(err, GrpcInboundError::Unimplemented(_)));
    }

    /// @covers: map_handler_error — ExecutionFailed maps to Internal.
    #[test]
    fn test_map_handler_error_execution_failed_maps_to_internal() {
        assert!(matches!(super::map_handler_error(HandlerError::ExecutionFailed("x".into())), GrpcInboundError::Internal(_)));
    }
}
