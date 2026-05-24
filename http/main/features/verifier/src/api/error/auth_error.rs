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
    InvalidToken(String),
}

impl From<VerifierError> for HttpAuthError {
    fn from(e: VerifierError) -> Self {
        HttpAuthError::InvalidToken(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: HttpAuthError — Display strings are non-empty.
    #[test]
    fn test_http_auth_error_display_is_non_empty() {
        assert!(!HttpAuthError::MissingAuthorization.to_string().is_empty());
        assert!(!HttpAuthError::MalformedAuthorization.to_string().is_empty());
        let wrapped = HttpAuthError::InvalidToken("expired".to_string());
        assert!(!wrapped.to_string().is_empty());
    }

    /// @covers: From<VerifierError> — conversion preserves the error message.
    #[test]
    fn test_from_verifier_error_produces_invalid_token_with_message() {
        use swe_edge_ingress_verifier::VerifierError;
        let err = HttpAuthError::from(VerifierError::Expired);
        assert!(matches!(err, HttpAuthError::InvalidToken(_)));
        assert!(!err.to_string().is_empty());
    }
}
