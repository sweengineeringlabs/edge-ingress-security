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
