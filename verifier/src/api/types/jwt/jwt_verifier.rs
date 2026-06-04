//! `JwtVerifier` struct declaration and configuration entry point.

use jsonwebtoken::Validation;

use crate::api::error::VerifierError;

use super::JwtConfig;

/// JWT verifier supporting HS256, RS256, and ES256.
///
/// Validates signature, `exp`, `nbf`, and optionally `iss` and `aud`.
/// Construct via [`JwtVerifier::from_config`]. The verifier implements
/// [`TokenVerifier`](crate::TokenVerifier) so it can be boxed and stored
/// as `Arc<dyn TokenVerifier>` for use with [`BearerTokenInterceptor`].
///
/// [`BearerTokenInterceptor`]: swe_edge_ingress_grpc_verifier::BearerTokenInterceptor
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier, TokenVerifier};
///
/// let config = JwtConfig {
///     key: JwtKey::Hs256 { secret: b"super-secret-key-32-bytes-long!!".to_vec() },
///     required_issuer: Some("https://auth.example.com".to_string()),
///     required_audience: None,
///     leeway_seconds: 0,
/// };
///
/// let verifier = JwtVerifier::from_config(&config).expect("valid key material");
/// // In tests, use a real signed token from your identity provider.
/// // verifier.verify("eyJ...").unwrap();
/// ```
pub struct JwtVerifier {
    pub(crate) key: jsonwebtoken::DecodingKey,
    pub(crate) validation: Validation,
}

impl JwtVerifier {
    /// Construct from [`JwtConfig`].
    ///
    /// Parses key material eagerly — failures surface at startup, not at
    /// request time. Returns [`VerifierError::Config`] if the key PEM is
    /// malformed or the algorithm is unsupported.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier};
    ///
    /// let cfg = JwtConfig {
    ///     key: JwtKey::Hs256 { secret: b"at-least-32-bytes-for-hs256-here".to_vec() },
    ///     required_issuer: None,
    ///     required_audience: None,
    ///     leeway_seconds: 0,
    /// };
    /// let verifier = JwtVerifier::from_config(&cfg).expect("valid config");
    /// ```
    pub fn from_config(config: &JwtConfig) -> Result<Self, VerifierError> {
        let (key, algorithm) =
            crate::core::jwt::DefaultJwtVerifier::build_decoding_key(&config.key)?;

        let mut validation = Validation::new(algorithm);
        validation.leeway = config.leeway_seconds;

        if let Some(ref iss) = config.required_issuer {
            validation.set_issuer(&[iss]);
        }

        if let Some(ref aud) = config.required_audience {
            validation.set_audience(&[aud]);
        } else {
            validation.validate_aud = false;
        }

        Ok(Self { key, validation })
    }
}
