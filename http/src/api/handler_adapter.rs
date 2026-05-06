//! Bridge between [`edge_domain::Handler`] and the HTTP inbound port.
//!
//! `HttpHandlerAdapter<Req, Resp>` wraps an `Arc<dyn Handler<Req, Resp>>`
//! together with a decode/encode pair and exposes the wrapped unit as a
//! [`Handler<HttpRequest, HttpResponse>`].  This lets a single
//! [`HandlerRegistry<HttpRequest, HttpResponse>`](edge_domain::HandlerRegistry)
//! hold dispatch entries for every HTTP route served by the server,
//! regardless of the underlying typed request/response pair.

use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError};

use crate::api::port::http_inbound::HttpInboundError;
use crate::api::value_object::{HttpRequest, HttpResponse};

/// Decodes a typed request from an inbound [`HttpRequest`].
///
/// Return [`HttpInboundError::InvalidInput`] when the request cannot be
/// decoded — that surfaces as a 400 response on the wire.
pub type HttpDecodeFn<Req> = fn(&HttpRequest) -> Result<Req, HttpInboundError>;

/// Encodes a typed response into an [`HttpResponse`].
pub type HttpEncodeFn<Resp> = fn(Resp) -> HttpResponse;

/// Adapter that exposes a typed [`Handler<Req, Resp>`] as a
/// [`Handler<HttpRequest, HttpResponse>`] for registration in a
/// [`HandlerRegistry`](edge_domain::HandlerRegistry).
///
/// `id` and `pattern` are forwarded verbatim from the inner handler.
/// By convention `pattern` is the URL path pattern (e.g. `"/users/:id"`).
pub struct HttpHandlerAdapter<Req, Resp>
where
    Req:  Send + 'static,
    Resp: Send + 'static,
{
    inner:  Arc<dyn Handler<Req, Resp>>,
    decode: HttpDecodeFn<Req>,
    encode: HttpEncodeFn<Resp>,
}

impl<Req, Resp> HttpHandlerAdapter<Req, Resp>
where
    Req:  Send + 'static,
    Resp: Send + 'static,
{
    /// Construct a new adapter from an inner typed handler and a
    /// decode/encode pair.
    pub fn new(
        inner:  Arc<dyn Handler<Req, Resp>>,
        decode: HttpDecodeFn<Req>,
        encode: HttpEncodeFn<Resp>,
    ) -> Self {
        Self { inner, decode, encode }
    }

    /// Borrow the inner typed handler.
    pub fn inner(&self) -> &Arc<dyn Handler<Req, Resp>> { &self.inner }
}

#[async_trait]
impl<Req, Resp> Handler<HttpRequest, HttpResponse> for HttpHandlerAdapter<Req, Resp>
where
    Req:  Send + 'static,
    Resp: Send + 'static,
{
    fn id(&self) -> &str { self.inner.id() }

    fn pattern(&self) -> &str { self.inner.pattern() }

    async fn execute(&self, req: HttpRequest) -> Result<HttpResponse, HandlerError> {
        let typed = (self.decode)(&req)
            .map_err(|e| HandlerError::InvalidRequest(e.to_string()))?;
        let resp = self.inner.execute(typed).await?;
        Ok((self.encode)(resp))
    }

    async fn health_check(&self) -> bool { self.inner.health_check().await }

    fn as_any(&self) -> &dyn Any { self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct GetUserReq { id: u32 }

    #[derive(Debug, PartialEq)]
    struct GetUserResp { name: String }

    fn decode_get_user(req: &HttpRequest) -> Result<GetUserReq, HttpInboundError> {
        req.query
            .get("id")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|id| GetUserReq { id })
            .ok_or_else(|| HttpInboundError::InvalidInput("missing id query param".into()))
    }

    fn encode_get_user(resp: GetUserResp) -> HttpResponse {
        HttpResponse::new(200, resp.name.into_bytes())
    }

    struct EchoUserHandler;

    #[async_trait]
    impl Handler<GetUserReq, GetUserResp> for EchoUserHandler {
        fn id(&self) -> &str { "get-user" }
        fn pattern(&self) -> &str { "/users" }
        async fn execute(&self, req: GetUserReq) -> Result<GetUserResp, HandlerError> {
            Ok(GetUserResp { name: format!("user-{}", req.id) })
        }
        async fn health_check(&self) -> bool { true }
        fn as_any(&self) -> &dyn Any { self }
    }

    struct FailingHandler;

    #[async_trait]
    impl Handler<GetUserReq, GetUserResp> for FailingHandler {
        fn id(&self) -> &str { "fail-user" }
        fn pattern(&self) -> &str { "/fail" }
        async fn execute(&self, _: GetUserReq) -> Result<GetUserResp, HandlerError> {
            Err(HandlerError::ExecutionFailed("boom".into()))
        }
        async fn health_check(&self) -> bool { false }
        fn as_any(&self) -> &dyn Any { self }
    }

    /// @covers: HttpHandlerAdapter::new — id and pattern forwarded from inner.
    #[tokio::test]
    async fn test_new_forwards_id_and_pattern_from_inner_handler() {
        let adapter = HttpHandlerAdapter::new(
            Arc::new(EchoUserHandler),
            decode_get_user,
            encode_get_user,
        );
        assert_eq!(adapter.id(),      "get-user");
        assert_eq!(adapter.pattern(), "/users");
    }

    /// @covers: HttpHandlerAdapter::execute — decodes, runs inner, encodes.
    #[tokio::test]
    async fn test_execute_decodes_invokes_inner_and_encodes_response() {
        let adapter: HttpHandlerAdapter<GetUserReq, GetUserResp> = HttpHandlerAdapter::new(
            Arc::new(EchoUserHandler),
            decode_get_user,
            encode_get_user,
        );
        let mut req = HttpRequest::get("/users");
        req.query.insert("id".into(), "42".into());
        let resp = (&adapter as &dyn Handler<HttpRequest, HttpResponse>)
            .execute(req).await.expect("execute ok");
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body,   b"user-42");
    }

    /// @covers: HttpHandlerAdapter::execute — decode failure surfaces as InvalidRequest.
    #[tokio::test]
    async fn test_execute_returns_invalid_request_when_decode_fails() {
        let adapter: HttpHandlerAdapter<GetUserReq, GetUserResp> = HttpHandlerAdapter::new(
            Arc::new(EchoUserHandler),
            decode_get_user,
            encode_get_user,
        );
        let req = HttpRequest::get("/users"); // no id query param
        let err = (&adapter as &dyn Handler<HttpRequest, HttpResponse>)
            .execute(req).await.expect_err("decode failure must error");
        assert!(matches!(err, HandlerError::InvalidRequest(_)));
    }

    /// @covers: HttpHandlerAdapter::execute — inner handler error propagates.
    #[tokio::test]
    async fn test_execute_propagates_inner_handler_error() {
        let adapter: HttpHandlerAdapter<GetUserReq, GetUserResp> = HttpHandlerAdapter::new(
            Arc::new(FailingHandler),
            decode_get_user,
            encode_get_user,
        );
        let mut req = HttpRequest::get("/fail");
        req.query.insert("id".into(), "1".into());
        let err = (&adapter as &dyn Handler<HttpRequest, HttpResponse>)
            .execute(req).await.expect_err("handler failure must error");
        assert!(matches!(err, HandlerError::ExecutionFailed(_)));
    }

    /// @covers: HttpHandlerAdapter::health_check — forwarded from inner.
    #[tokio::test]
    async fn test_health_check_forwards_to_inner_handler() {
        let healthy   = HttpHandlerAdapter::new(Arc::new(EchoUserHandler), decode_get_user, encode_get_user);
        let unhealthy = HttpHandlerAdapter::new(Arc::new(FailingHandler),  decode_get_user, encode_get_user);
        assert!( (&healthy   as &dyn Handler<HttpRequest, HttpResponse>).health_check().await);
        assert!(!(&unhealthy as &dyn Handler<HttpRequest, HttpResponse>).health_check().await);
    }

    /// @covers: HttpHandlerAdapter::inner — exposes wrapped Arc<Handler>.
    #[tokio::test]
    async fn test_inner_returns_wrapped_handler() {
        let adapter: HttpHandlerAdapter<GetUserReq, GetUserResp> = HttpHandlerAdapter::new(
            Arc::new(EchoUserHandler),
            decode_get_user,
            encode_get_user,
        );
        assert_eq!(adapter.inner().id(), "get-user");
    }
}
