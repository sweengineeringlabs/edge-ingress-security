//! Domain error type for bearer token verification failures.

use thiserror::Error;

/// Domain-level error for bearer token verification operations.
#[derive(Debug, Error)]
pub enum VerifierError {
    /// Token verification failed due to invalid signature or format.
    #[error("verification failed: {0}")]
    VerificationFailed(String),
    /// Bearer credentials are missing from the request.
    #[error("bearer credentials missing")]
    MissingCredentials,
    /// Bearer credentials are malformed or incomplete.
    #[error("malformed bearer credentials")]
    MalformedCredentials,
    /// Configuration error in verification setup.
    #[error("verification configuration error: {0}")]
    ConfigurationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_error_verification_failed() {
        let err = VerifierError::VerificationFailed("invalid signature".into());
        assert!(err.to_string().contains("verification failed"));
    }

    #[test]
    fn test_verifier_error_missing_credentials() {
        let err = VerifierError::MissingCredentials;
        assert!(err.to_string().contains("missing"));
    }

    #[test]
    fn test_verifier_error_malformed_credentials() {
        let err = VerifierError::MalformedCredentials;
        assert!(err.to_string().contains("malformed"));
    }

    #[test]
    fn test_verifier_error_configuration_error() {
        let err = VerifierError::ConfigurationError("invalid issuer".into());
        assert!(err.to_string().contains("configuration"));
    }
}
