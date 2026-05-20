//! Result type alias for HTTP inbound operations.

use crate::api::port::http_inbound_error::HttpInboundError;

/// Result type for HTTP inbound operations.
pub type HttpInboundResult<T> = Result<T, HttpInboundError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_inbound_result_ok_wraps_value() {
        let r: HttpInboundResult<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn test_http_inbound_result_err_wraps_error() {
        let r: HttpInboundResult<u32> = Err(HttpInboundError::Internal("oops".into()));
        assert!(r.is_err());
    }
}
