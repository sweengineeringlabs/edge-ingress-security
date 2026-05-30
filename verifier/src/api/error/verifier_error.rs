//! `VerifierError` — error type for all verifier operations.

/// Errors returned by [`TokenVerifier`](super::token_verifier::TokenVerifier)
/// and [`ApiKeyVerifier`](super::api_key_verifier::ApiKeyVerifier).
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
