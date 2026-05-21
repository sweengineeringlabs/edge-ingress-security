//! Bridge between edge_domain::Handler and the gRPC inbound port.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError, RequestContext};

use crate::api::port::grpc_inbound::GrpcInboundError;

use crate::api::handler::decode_fn::DecodeFn;
use crate::api::handler::encode_fn::EncodeFn;

/// Adapter that exposes a typed [`Handler<Req, Resp>`] as a
/// [`Handler<Vec<u8>, Vec<u8>>`] for registration in a single
/// byte-oriented [`HandlerRegistry`](edge_domain::HandlerRegistry).
///
/// `id` is forwarded verbatim from the inner handler — by convention
/// it is the fully-qualified gRPC method path
/// (e.g. `"/pkg.Service/Method"`).
pub struct GrpcHandlerAdapter<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    inner: Arc<dyn Handler<Req, Resp>>,
    decode: DecodeFn<Req>,
    encode: EncodeFn<Resp>,
}

impl<Req, Resp> GrpcHandlerAdapter<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Construct a new adapter from an inner typed handler and a
    /// decode/encode pair.
    pub fn new(
        inner: Arc<dyn Handler<Req, Resp>>,
        decode: DecodeFn<Req>,
        encode: EncodeFn<Resp>,
    ) -> Self {
        Self {
            inner,
            decode,
            encode,
        }
    }

    /// Borrow the inner typed handler — useful for administrative
    /// tooling that wants to interrogate the underlying domain unit.
    pub fn inner(&self) -> &Arc<dyn Handler<Req, Resp>> {
        &self.inner
    }
}

#[async_trait]
impl<Req, Resp> Handler<Vec<u8>, Vec<u8>> for GrpcHandlerAdapter<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn pattern(&self) -> &str {
        self.inner.pattern()
    }

    async fn execute(&self, req: Vec<u8>) -> Result<Vec<u8>, HandlerError> {
        self.execute_with_context(req, RequestContext::unauthenticated())
            .await
    }

    async fn execute_with_context(
        &self,
        req: Vec<u8>,
        ctx: RequestContext,
    ) -> Result<Vec<u8>, HandlerError> {
        let typed = (self.decode)(&req).map_err(|e| match e {
            GrpcInboundError::InvalidArgument(msg) => HandlerError::InvalidRequest(msg),
            GrpcInboundError::Status(_, msg) => HandlerError::InvalidRequest(msg),
            other => HandlerError::InvalidRequest(other.to_string()),
        })?;
        let resp = self.inner.execute_with_context(typed, ctx).await?;
        Ok((self.encode)(&resp))
    }

    async fn health_check(&self) -> bool {
        self.inner.health_check().await
    }
}

#[cfg(test)]
mod tests {
    use edge_domain::RequestContext;

    use super::*;

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
        let value = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(TestReq { value })
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

    struct UnhealthyHandler;

    #[async_trait::async_trait]
    impl Handler<TestReq, TestResp> for UnhealthyHandler {
        fn id(&self) -> &str {
            "/pkg.Service/Sick"
        }
        fn pattern(&self) -> &str {
            "test"
        }
        async fn execute(&self, _: TestReq) -> Result<TestResp, HandlerError> {
            Err(HandlerError::Unhealthy)
        }
        async fn health_check(&self) -> bool {
            false
        }
    }

    #[tokio::test]
    async fn test_new_forwards_id_and_pattern_from_inner_handler() {
        let adapter: GrpcHandlerAdapter<TestReq, TestResp> =
            GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp);
        assert_eq!(adapter.id(), "/pkg.Service/Double");
        assert_eq!(adapter.pattern(), "test");
    }

    #[tokio::test]
    async fn test_execute_decodes_invokes_inner_and_encodes_response() {
        let adapter: GrpcHandlerAdapter<TestReq, TestResp> =
            GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp);
        let req_bytes = 21u32.to_be_bytes().to_vec();
        let resp = (&adapter as &dyn Handler<Vec<u8>, Vec<u8>>)
            .execute_with_context(req_bytes, RequestContext::unauthenticated())
            .await
            .expect("execute");
        let value = u32::from_be_bytes([resp[0], resp[1], resp[2], resp[3]]);
        assert_eq!(value, 42, "DoublingHandler should multiply input by 2");
    }

    #[tokio::test]
    async fn test_execute_returns_invalid_request_when_decode_fails() {
        let adapter: GrpcHandlerAdapter<TestReq, TestResp> =
            GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp);
        let bad_bytes = vec![1u8, 2, 3];
        let err = (&adapter as &dyn Handler<Vec<u8>, Vec<u8>>)
            .execute_with_context(bad_bytes, RequestContext::unauthenticated())
            .await
            .expect_err("decode failure must error");
        match err {
            HandlerError::InvalidRequest(msg) => {
                assert!(
                    msg.contains("4 bytes"),
                    "msg should describe decode failure: {msg}"
                );
            }
            other => panic!("expected InvalidRequest, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_execute_propagates_inner_handler_error() {
        let adapter: GrpcHandlerAdapter<TestReq, TestResp> = GrpcHandlerAdapter::new(
            Arc::new(UnhealthyHandler),
            decode_test_req,
            encode_test_resp,
        );
        let req_bytes = 1u32.to_be_bytes().to_vec();
        let err = (&adapter as &dyn Handler<Vec<u8>, Vec<u8>>)
            .execute_with_context(req_bytes, RequestContext::unauthenticated())
            .await
            .expect_err("unhealthy must error");
        assert!(matches!(err, HandlerError::Unhealthy), "got: {err:?}");
    }

    #[tokio::test]
    async fn test_health_check_forwards_to_inner_handler() {
        let healthy: GrpcHandlerAdapter<TestReq, TestResp> =
            GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp);
        let unhealthy: GrpcHandlerAdapter<TestReq, TestResp> = GrpcHandlerAdapter::new(
            Arc::new(UnhealthyHandler),
            decode_test_req,
            encode_test_resp,
        );
        assert!(
            (&healthy as &dyn Handler<Vec<u8>, Vec<u8>>)
                .health_check()
                .await
        );
        assert!(
            !(&unhealthy as &dyn Handler<Vec<u8>, Vec<u8>>)
                .health_check()
                .await
        );
    }

    #[tokio::test]
    async fn test_inner_returns_wrapped_handler() {
        let adapter: GrpcHandlerAdapter<TestReq, TestResp> =
            GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp);
        assert_eq!(adapter.inner().id(), "/pkg.Service/Double");
    }

    /// @covers: inner
    #[test]
    fn test_inner_exposes_arc_to_inner_handler() {
        let adapter: GrpcHandlerAdapter<TestReq, TestResp> =
            GrpcHandlerAdapter::new(Arc::new(DoublingHandler), decode_test_req, encode_test_resp);
        // Verify inner() returns the same Arc
        let inner = adapter.inner();
        assert_eq!(inner.id(), "/pkg.Service/Double");
    }
}
