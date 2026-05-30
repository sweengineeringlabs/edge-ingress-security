//! `JwtVerifier` struct declaration and configuration entry point.

use jsonwebtoken::Validation;

use crate::api::error::VerifierError;

use super::JwtConfig;

/// JWT verifier supporting HS256, RS256, and ES256.
///
/// Validates signature, `exp`, `nbf`, and optionally `iss` and `aud`.
/// Construct via [`JwtVerifier::from_config`].
pub struct JwtVerifier {
    pub(crate) key: jsonwebtoken::DecodingKey,
    pub(crate) validation: Validation,
}

impl JwtVerifier {
    /// Construct from [`JwtConfig`].
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
