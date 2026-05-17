//! Error type returned by the inbound bearer interceptor.

/// Reasons the bearer inbound interceptor rejects a call.
#[derive(Debug, thiserror::Error)]
pub enum BearerAuthError {
    /// The `authorization` header was absent.
    #[error("missing authorization header")]
    MissingHeader,
    /// The header was present but not in `Bearer <token>` format.
    #[error("malformed authorization header")]
    MalformedHeader,
    /// JWT signature, expiry, audience, or issuer check failed.
    #[error("invalid bearer token: {0}")]
    InvalidToken(#[source] jsonwebtoken::errors::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: BearerAuthError implements std::error::Error.
    #[test]
    fn test_bearer_auth_error_implements_std_error() {
        let e = BearerAuthError::MissingHeader;
        let _: &dyn std::error::Error = &e;
        assert!(e.to_string().contains("missing"));
    }

    /// @covers: MalformedHeader — display message.
    #[test]
    fn test_malformed_header_display_message() {
        let e = BearerAuthError::MalformedHeader;
        assert!(e.to_string().contains("malformed"));
    }
}
