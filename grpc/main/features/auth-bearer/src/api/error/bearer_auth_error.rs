//! Error type returned by the inbound bearer interceptor.

/// Reasons the bearer inbound interceptor rejects a call.
///
/// All variants map to gRPC status `Unauthenticated` (status code 16).
/// The error message is forwarded to the caller in the `grpc-status-message`
/// trailer — avoid including internal details in `InvalidToken`.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_auth_bearer::BearerAuthError;
///
/// let err = BearerAuthError::MissingHeader;
/// assert!(err.to_string().contains("missing"));
///
/// let err = BearerAuthError::MalformedHeader;
/// assert!(err.to_string().contains("malformed"));
/// ```
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
