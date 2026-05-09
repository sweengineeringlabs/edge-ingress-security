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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: VerifierError — Display messages are non-empty.
    #[test]
    fn test_verifier_error_display_messages_are_non_empty() {
        assert!(!VerifierError::Expired.to_string().is_empty());
        assert!(!VerifierError::NotYetValid.to_string().is_empty());
        assert!(!VerifierError::UnknownApiKey.to_string().is_empty());
        assert!(!VerifierError::Invalid("bad".into()).to_string().is_empty());
        assert!(!VerifierError::ClaimMismatch("iss".into()).to_string().is_empty());
        assert!(!VerifierError::Config("key".into()).to_string().is_empty());
    }
}
