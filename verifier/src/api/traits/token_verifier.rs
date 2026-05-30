//! `TokenVerifier` — protocol-agnostic JWT verification trait.

use crate::api::error::VerifierError;
use crate::api::types::Claims;

/// Verifies an inbound bearer token string and returns its claims.
///
/// Implementations decide the algorithm (HS256, RS256, ES256), key
/// material, and which claims to enforce.  The trait is object-safe so
/// callers can hold `Arc<dyn TokenVerifier>`.
pub trait TokenVerifier: Send + Sync {
    /// Verify `token` and return the extracted [`Claims`] on success.
    fn verify(&self, token: &str) -> Result<Claims, VerifierError>;
}
