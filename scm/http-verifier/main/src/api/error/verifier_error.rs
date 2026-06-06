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
