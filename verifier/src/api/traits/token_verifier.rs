//! `TokenVerifier` — protocol-agnostic JWT verification trait.

use crate::api::error::VerifierError;
use crate::api::vo::Claims;

/// Verifies an inbound bearer token string and returns its claims.
///
/// Implementations decide the algorithm (HS256, RS256, ES256), key
/// material, and which claims to enforce. The trait is object-safe so
/// callers can store `Arc<dyn TokenVerifier>` and swap implementations
/// (e.g. swap a noop stub in tests for a real JWT verifier in production).
///
/// The built-in implementation is [`JwtVerifier`](crate::JwtVerifier).
///
/// # Examples
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
///
/// /// Stub verifier that accepts any token in tests.
/// struct AlwaysAllow;
/// impl TokenVerifier for AlwaysAllow {
///     fn verify(&self, _token: &str) -> Result<Claims, VerifierError> {
///         Ok(Claims::builder().sub("test-user").build())
///     }
/// }
///
/// let verifier: Arc<dyn TokenVerifier> = Arc::new(AlwaysAllow);
/// let claims = verifier.verify("any.token.here").unwrap();
/// assert_eq!(claims.sub.as_deref(), Some("test-user"));
/// ```
pub trait TokenVerifier: Send + Sync {
    /// Verify `token` and return the extracted [`Claims`] on success.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier, TokenVerifier};
    ///
    /// # let cfg = JwtConfig { key: JwtKey::Hs256 { secret: vec![] }, required_issuer: None, required_audience: None, leeway_seconds: 0 };
    /// let verifier = JwtVerifier::from_config(&cfg).unwrap();
    /// // verifier.verify("eyJ...") returns Ok(Claims) or Err(VerifierError::Expired) etc.
    /// ```
    fn verify(&self, token: &str) -> Result<Claims, VerifierError>;
}
