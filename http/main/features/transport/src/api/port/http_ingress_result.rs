//! Result type alias for HTTP inbound operations.

use crate::api::port::http_ingress_error::HttpIngressError;

/// Result type for HTTP inbound operations.
pub type HttpIngressResult<T> = Result<T, HttpIngressError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_ingress_result_ok_wraps_value() {
        let r: HttpIngressResult<u32> = Ok(42);
        assert!(matches!(r, Ok(42)));
    }

    #[test]
    fn test_http_ingress_result_err_wraps_error() {
        let r: HttpIngressResult<u32> = Err(HttpIngressError::Internal("oops".into()));
        assert!(r.is_err());
    }
}
