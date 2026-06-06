//! `VerifierError` — error type for all verifier operations.

/// Errors returned by [`TokenVerifier`](crate::api::traits::token_verifier::TokenVerifier)
/// and [`ApiKeyVerifier`](crate::api::types::api_key_verifier::ApiKeyVerifier).
///
/// All variants except [`Config`](VerifierError::Config) indicate a caller
/// error (bad token, expired token, wrong key) — return HTTP 401 to the client.
/// `Config` indicates a server misconfiguration — return HTTP 500 and page on-call.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_verifier::VerifierError;
///
/// let err = VerifierError::Expired;
/// assert!(err.to_string().contains("expired"));
///
/// // Map to HTTP status codes.
/// let status = match err {
///     VerifierError::Invalid(_)
///     | VerifierError::Expired
///     | VerifierError::NotYetValid
///     | VerifierError::UnknownApiKey => 401,
///     VerifierError::ClaimMismatch(_) => 403,
///     VerifierError::Config(_) => 500,
/// };
/// assert_eq!(status, 401);
/// ```
#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    /// Token signature or structure is invalid.
    #[error("invalid token: {0}")]
    Invalid(String),
    /// Token has expired.
    #[error("token expired")]
    Expired,
    /// `nbf` claim is in the future.
    #[error("token not yet valid")]
    NotYetValid,
    /// Required claim (`iss`, `aud`, `sub`) does not match expected value.
    #[error("claim mismatch: {0}")]
    ClaimMismatch(String),
    /// API key was not recognised.
    #[error("unknown api key")]
    UnknownApiKey,
    /// Verifier configuration is invalid (wrong key format, unsupported algorithm, etc.).
    #[error("verifier config error: {0}")]
    Config(String),
}

impl From<VerifierError> for edge_domain::HandlerError {
    fn from(e: VerifierError) -> Self {
        match e {
            VerifierError::Invalid(_)
            | VerifierError::Expired
            | VerifierError::NotYetValid
            | VerifierError::UnknownApiKey => {
                edge_domain::HandlerError::Unauthorized(e.to_string())
            }
            VerifierError::ClaimMismatch(_) => {
                edge_domain::HandlerError::PermissionDenied(e.to_string())
            }
            VerifierError::Config(_) => edge_domain::HandlerError::ExecutionFailed(e.to_string()),
        }
    }
}
