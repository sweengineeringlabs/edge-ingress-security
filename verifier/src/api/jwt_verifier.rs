//! `JwtVerifier` struct declaration and configuration entry point.

use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use crate::api::jwt_config::{JwtConfig, JwtKey};
use crate::api::verifier_error::VerifierError;

/// JWT verifier supporting HS256, RS256, and ES256.
///
/// Validates signature, `exp`, `nbf`, and optionally `iss` and `aud`.
/// Construct via [`JwtVerifier::from_config`].
pub struct JwtVerifier {
    pub(crate) key:        DecodingKey,
    pub(crate) validation: Validation,
}

impl JwtVerifier {
    /// Construct from [`JwtConfig`].
    pub fn from_config(config: &JwtConfig) -> Result<Self, VerifierError> {
        let (key, algorithm) = build_decoding_key(&config.key)?;

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

pub(crate) fn build_decoding_key(key: &JwtKey) -> Result<(DecodingKey, Algorithm), VerifierError> {
    match key {
        JwtKey::Hs256 { secret } => {
            Ok((DecodingKey::from_secret(secret.as_slice()), Algorithm::HS256))
        }
        JwtKey::Rs256 { public_pem } => {
            let k = DecodingKey::from_rsa_pem(public_pem.as_bytes())
                .map_err(|e| VerifierError::Config(e.to_string()))?;
            Ok((k, Algorithm::RS256))
        }
        JwtKey::Es256 { public_pem } => {
            let k = DecodingKey::from_ec_pem(public_pem.as_bytes())
                .map_err(|e| VerifierError::Config(e.to_string()))?;
            Ok((k, Algorithm::ES256))
        }
    }
}

pub(crate) fn map_jwt_error(e: jsonwebtoken::errors::Error) -> VerifierError {
    use jsonwebtoken::errors::ErrorKind;
    match e.kind() {
        ErrorKind::ExpiredSignature  => VerifierError::Expired,
        ErrorKind::ImmatureSignature => VerifierError::NotYetValid,
        ErrorKind::InvalidIssuer     => VerifierError::ClaimMismatch("iss".into()),
        ErrorKind::InvalidAudience   => VerifierError::ClaimMismatch("aud".into()),
        ErrorKind::InvalidSubject    => VerifierError::ClaimMismatch("sub".into()),
        _                            => VerifierError::Invalid(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::jwt_config::{JwtConfig, JwtKey};

    fn config(secret: &[u8]) -> JwtConfig {
        JwtConfig {
            key:               JwtKey::Hs256 { secret: secret.to_vec() },
            required_issuer:   None,
            required_audience: None,
            leeway_seconds:    0,
        }
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_succeeds_for_hs256() {
        assert!(JwtVerifier::from_config(&config(b"secret")).is_ok());
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_fails_for_invalid_rs256_pem() {
        let cfg = JwtConfig {
            key:               JwtKey::Rs256 { public_pem: "not-a-pem".into() },
            required_issuer:   None,
            required_audience: None,
            leeway_seconds:    0,
        };
        assert!(JwtVerifier::from_config(&cfg).is_err());
    }
}
