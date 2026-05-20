//! Error type for HTTP inbound operations.

/// Error type for HTTP inbound operations.
#[derive(Debug, thiserror::Error)]
pub enum HttpInboundError {
    /// Internal server error.
    #[error("internal: {0}")]
    Internal(String),
    /// Requested resource not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// Request input failed validation.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Upstream service unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// Operation timed out.
    #[error("timeout: {0}")]
    Timeout(String),
    /// Caller is not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// Caller lacks permission.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// The operation conflicts with existing state.
    #[error("conflict: {0}")]
    Conflict(String),
    /// The handler does not support the requested operation. Maps to HTTP 405.
    #[error("method not allowed: {0}")]
    MethodNotAllowed(String),
    /// The request is valid but rejected by a business rule or precondition. Maps to HTTP 422.
    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_inbound_error_internal_formats_correctly() {
        let err = HttpInboundError::Internal("oops".into());
        assert!(err.to_string().contains("oops"));
    }
}
