//! `HttpAuthError` — reasons an inbound HTTP request failed authentication.

use swe_edge_ingress_verifier::VerifierError;
use thiserror::Error;

/// Errors produced by the bearer-token verification layer.
#[derive(Debug, Error)]
pub enum HttpAuthError {
    /// `Authorization` header is absent.
    #[error("missing Authorization header")]
    MissingAuthorization,
    /// Header is present but not a `Bearer <token>` value.
    #[error("malformed Authorization header: expected 'Bearer <token>'")]
    MalformedAuthorization,
    /// The token was rejected by the underlying verifier.
    #[error("invalid token: {0}")]
    InvalidToken(#[from] VerifierError),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: HttpAuthError — Display strings are non-empty.
    #[test]
    fn test_http_auth_error_display_is_non_empty() {
        assert!(!HttpAuthError::MissingAuthorization.to_string().is_empty());
        assert!(!HttpAuthError::MalformedAuthorization.to_string().is_empty());
        let wrapped = HttpAuthError::InvalidToken(VerifierError::Expired);
        assert!(!wrapped.to_string().is_empty());
    }
}
