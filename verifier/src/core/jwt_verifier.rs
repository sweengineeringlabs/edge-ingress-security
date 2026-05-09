//! `JwtVerifier` — concrete JWT verifier backed by `jsonwebtoken`.

use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use crate::api::claims::Claims;
use crate::api::jwt_config::{JwtConfig, JwtKey};
use crate::api::token_verifier::TokenVerifier;
use crate::api::verifier_error::VerifierError;

/// JWT verifier supporting HS256, RS256, and ES256.
///
/// Validates signature, `exp`, `nbf`, and optionally `iss` and `aud`.
/// Construct via [`JwtVerifier::from_config`].
pub struct JwtVerifier {
    key:        DecodingKey,
    validation: Validation,
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

fn build_decoding_key(key: &JwtKey) -> Result<(DecodingKey, Algorithm), VerifierError> {
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

impl TokenVerifier for JwtVerifier {
    fn verify(&self, token: &str) -> Result<Claims, VerifierError> {
        let data = jsonwebtoken::decode::<Claims>(token, &self.key, &self.validation)
            .map_err(|e| map_jwt_error(e))?;
        Ok(data.claims)
    }
}

fn map_jwt_error(e: jsonwebtoken::errors::Error) -> VerifierError {
    use jsonwebtoken::errors::ErrorKind;
    match e.kind() {
        ErrorKind::ExpiredSignature          => VerifierError::Expired,
        ErrorKind::ImmatureSignature         => VerifierError::NotYetValid,
        ErrorKind::InvalidIssuer             => VerifierError::ClaimMismatch("iss".into()),
        ErrorKind::InvalidAudience           => VerifierError::ClaimMismatch("aud".into()),
        ErrorKind::InvalidSubject            => VerifierError::ClaimMismatch("sub".into()),
        _                                    => VerifierError::Invalid(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::jwt_config::{JwtConfig, JwtKey};

    fn hs256_token(secret: &[u8], claims: &Claims, exp_offset: i64) -> String {
        use jsonwebtoken::{encode, EncodingKey, Header};
        #[derive(serde::Serialize)]
        struct Raw {
            sub: Option<String>,
            iss: Option<String>,
            exp: Option<u64>,
            nbf: Option<u64>,
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let raw = Raw {
            sub: claims.sub.clone(),
            iss: claims.iss.clone(),
            exp: Some((now as i64 + exp_offset) as u64),
            nbf: None,
        };
        encode(&Header::default(), &raw, &EncodingKey::from_secret(secret)).unwrap()
    }

    fn config(secret: &[u8]) -> JwtConfig {
        JwtConfig {
            key: JwtKey::Hs256 { secret: secret.to_vec() },
            required_issuer:   None,
            required_audience: None,
            leeway_seconds:    0,
        }
    }

    fn claims_with(sub: &str) -> Claims {
        serde_json::from_str(&format!(r#"{{"sub":"{sub}"}}"#)).unwrap()
    }

    /// @covers: from_config — builds verifier from HS256 config.
    #[test]
    fn test_from_config_succeeds_for_hs256() {
        assert!(JwtVerifier::from_config(&config(b"secret")).is_ok());
    }

    /// @covers: verify — valid HS256 token returns claims.
    #[test]
    fn test_verify_valid_hs256_token_returns_claims() {
        let secret = b"testsecret";
        let v = JwtVerifier::from_config(&config(secret)).unwrap();
        let token = hs256_token(secret, &claims_with("alice"), 3600);
        let claims = v.verify(&token).expect("valid token");
        assert_eq!(claims.sub.as_deref(), Some("alice"));
    }

    /// @covers: verify — wrong secret returns Invalid.
    #[test]
    fn test_verify_wrong_secret_returns_invalid() {
        let v = JwtVerifier::from_config(&config(b"correct")).unwrap();
        let token = hs256_token(b"wrong", &claims_with("bob"), 3600);
        let err = v.verify(&token).unwrap_err();
        assert!(matches!(err, VerifierError::Invalid(_)));
    }

    /// @covers: verify — expired token returns Expired.
    #[test]
    fn test_verify_expired_token_returns_expired_error() {
        let secret = b"key";
        let v = JwtVerifier::from_config(&config(secret)).unwrap();
        let token = hs256_token(secret, &claims_with("x"), -1);
        let err = v.verify(&token).unwrap_err();
        assert!(matches!(err, VerifierError::Expired), "{err:?}");
    }

    /// @covers: verify — malformed token returns Invalid.
    #[test]
    fn test_verify_malformed_token_returns_invalid() {
        let v = JwtVerifier::from_config(&config(b"key")).unwrap();
        assert!(matches!(v.verify("not.a.jwt"), Err(VerifierError::Invalid(_))));
    }
}
