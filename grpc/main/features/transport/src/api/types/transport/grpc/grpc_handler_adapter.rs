//! Bridge between edge_domain::Handler and the gRPC inbound port.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError, RequestContext};

use crate::api::port::grpc::GrpcIngressError;

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
            GrpcIngressError::InvalidArgument(msg) => HandlerError::InvalidRequest(msg),
            GrpcIngressError::Status(_, msg) => HandlerError::InvalidRequest(msg),
            other => HandlerError::InvalidRequest(other.to_string()),
        })?;
        let resp = self.inner.execute_with_context(typed, ctx).await?;
        Ok((self.encode)(&resp))
    }

    async fn health_check(&self) -> bool {
        self.inner.health_check().await
    }
}
