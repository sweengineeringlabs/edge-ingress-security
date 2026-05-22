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
