//! Encode function type alias for the HTTP adapter.

use crate::api::value_object::HttpResponse;

/// Encodes a typed response into an [`HttpResponse`].
pub type HttpEncodeFn<Resp> = fn(Resp) -> HttpResponse;

#[cfg(test)]
mod tests {
    use super::*;

    struct Pong {
        message: String,
    }

    fn encode_pong(resp: Pong) -> HttpResponse {
        HttpResponse::new(200, resp.message.into_bytes())
    }

    #[test]
    fn test_http_encode_fn_converts_response_to_http_response() {
        let f: HttpEncodeFn<Pong> = encode_pong;
        let resp = f(Pong {
            message: "pong".into(),
        });
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, b"pong");
    }

    #[test]
    fn test_http_encode_fn_is_compatible_with_fn_pointer_type() {
        let _: HttpEncodeFn<Pong> = encode_pong;
    }
}
