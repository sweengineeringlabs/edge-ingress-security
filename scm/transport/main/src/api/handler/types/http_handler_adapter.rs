//! Bridge between [`edge_domain::Handler`] and the HTTP inbound port.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError, RequestContext};

use crate::api::handler::traits::HttpDecodeFn;
use crate::api::handler::traits::HttpEncodeFn;
use crate::api::vo::{HttpRequest, HttpResponse};

/// Adapter that exposes a typed [`Handler<Req, Resp>`] as a
/// [`Handler<HttpRequest, HttpResponse>`] for registration in a
/// [`HandlerRegistry`](edge_domain::HandlerRegistry).
pub struct HttpHandlerAdapter<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    inner: Arc<dyn Handler<Req, Resp>>,
    decode: HttpDecodeFn<Req>,
    encode: HttpEncodeFn<Resp>,
}

impl<Req, Resp> HttpHandlerAdapter<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Construct a new adapter from an inner typed handler and a decode/encode pair.
    pub fn new(
        inner: Arc<dyn Handler<Req, Resp>>,
        decode: HttpDecodeFn<Req>,
        encode: HttpEncodeFn<Resp>,
    ) -> Self {
        Self {
            inner,
            decode,
            encode,
        }
    }

    /// Borrow the inner typed handler.
    pub fn inner(&self) -> &Arc<dyn Handler<Req, Resp>> {
        &self.inner
    }
}

#[async_trait]
impl<Req, Resp> Handler<HttpRequest, HttpResponse> for HttpHandlerAdapter<Req, Resp>
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

    async fn execute(&self, req: HttpRequest) -> Result<HttpResponse, HandlerError> {
        self.execute_with_context(req, RequestContext::unauthenticated())
            .await
    }

    async fn execute_with_context(
        &self,
        req: HttpRequest,
        ctx: RequestContext,
    ) -> Result<HttpResponse, HandlerError> {
        let typed = (self.decode)(&req).map_err(|e| HandlerError::InvalidRequest(e.to_string()))?;
        let resp = self.inner.execute_with_context(typed, ctx).await?;
        Ok((self.encode)(resp))
    }

    async fn health_check(&self) -> bool {
        self.inner.health_check().await
    }
}
