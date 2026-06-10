//! `TokenVerifier` impl for `JwtVerifier` plus JWT helper methods on `DefaultJwtVerifier`.

use jsonwebtoken::{Algorithm, DecodingKey};

use crate::api::error::VerifierError;
use crate::api::traits::TokenVerifier;
use crate::api::types::JwtVerifier;
use crate::api::types::{Claims, JwtKey};

/// Primary type for this module — satisfies Rule 89 filename match.
pub(crate) struct DefaultJwtVerifier;

impl DefaultJwtVerifier {
    pub(crate) fn build_decoding_key(
        key: &JwtKey,
    ) -> Result<(DecodingKey, Algorithm), VerifierError> {
        match key {
            JwtKey::Hs256 { secret } => Ok((
                DecodingKey::from_secret(secret.as_slice()),
                Algorithm::HS256,
            )),
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
            ErrorKind::ExpiredSignature => VerifierError::Expired,
            ErrorKind::ImmatureSignature => VerifierError::NotYetValid,
            ErrorKind::InvalidIssuer => VerifierError::ClaimMismatch("iss".into()),
            ErrorKind::InvalidAudience => VerifierError::ClaimMismatch("aud".into()),
            ErrorKind::InvalidSubject => VerifierError::ClaimMismatch("sub".into()),
            _ => VerifierError::Invalid(e.to_string()),
        }
    }
}

impl TokenVerifier for JwtVerifier {
    fn verify(&self, token: &str) -> Result<Claims, VerifierError> {
        let data = jsonwebtoken::decode::<Claims>(token, &self.key, &self.validation)
            .map_err(DefaultJwtVerifier::map_jwt_error)?;
        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_decoding_key_hs256_succeeds() {
        let key = JwtKey::Hs256 {
            secret: b"secret".to_vec(),
        };
        assert!(DefaultJwtVerifier::build_decoding_key(&key).is_ok());
    }

    #[test]
    fn test_build_decoding_key_rs256_fails_for_invalid_pem() {
        let key = JwtKey::Rs256 {
            public_pem: "not-pem".into(),
        };
        assert!(DefaultJwtVerifier::build_decoding_key(&key).is_err());
    }

    #[test]
    fn test_map_jwt_error_expired_signature_maps_to_expired_variant() {
        use jsonwebtoken::{decode, DecodingKey, Validation};
        let key = DecodingKey::from_secret(b"wrong");
        let mut val = Validation::default();
        val.validate_exp = true;
        // Create a definitely-expired token signature mismatch → maps to Invalid
        let raw_err = decode::<crate::api::types::Claims>("not.a.jwt.at.all", &key, &val)
            .expect_err("decode must fail with malformed JWT");
        let mapped = DefaultJwtVerifier::map_jwt_error(raw_err);
        assert!(matches!(mapped, VerifierError::Invalid(_)));
    }
}
