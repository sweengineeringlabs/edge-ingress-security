//! Decode function type alias for the HTTP adapter.

use crate::api::port::http_inbound_error::HttpInboundError;
use crate::api::value_object::HttpRequest;

/// Decodes a typed request from an inbound [`HttpRequest`].
pub type HttpDecodeFn<Req> = fn(&HttpRequest) -> Result<Req, HttpInboundError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Ping;

    fn decode_ping(req: &HttpRequest) -> Result<Ping, HttpInboundError> {
        if req.url.contains("ping") {
            Ok(Ping)
        } else {
            Err(HttpInboundError::InvalidInput("not a ping".into()))
        }
    }

    #[test]
    fn test_http_decode_fn_returns_ok_for_valid_request() {
        let f: HttpDecodeFn<Ping> = decode_ping;
        let req = HttpRequest::get("/ping");
        assert!(f(&req).is_ok());
    }

    #[test]
    fn test_http_decode_fn_returns_err_for_invalid_request() {
        let f: HttpDecodeFn<Ping> = decode_ping;
        let req = HttpRequest::get("/other");
        assert!(f(&req).is_err());
    }
}
