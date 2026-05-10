//! `TokenVerifier` impl for `JwtVerifier`.

use crate::api::claims::Claims;
use crate::api::jwt_verifier::{map_jwt_error, JwtVerifier};
use crate::api::token_verifier::TokenVerifier;
use crate::api::verifier_error::VerifierError;

impl TokenVerifier for JwtVerifier {
    fn verify(&self, token: &str) -> Result<Claims, VerifierError> {
        let data = jsonwebtoken::decode::<Claims>(token, &self.key, &self.validation)
            .map_err(map_jwt_error)?;
        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::jwt_config::{JwtConfig, JwtKey};

    fn hs256_token(secret: &[u8], sub: &str, exp_offset: i64) -> String {
        use jsonwebtoken::{encode, EncodingKey, Header};
        #[derive(serde::Serialize)]
        struct Raw { sub: String, exp: u64 }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let raw = Raw { sub: sub.into(), exp: (now as i64 + exp_offset) as u64 };
        encode(&Header::default(), &raw, &EncodingKey::from_secret(secret)).unwrap()
    }

    fn config(secret: &[u8]) -> JwtConfig {
        JwtConfig {
            key:               JwtKey::Hs256 { secret: secret.to_vec() },
            required_issuer:   None,
            required_audience: None,
            leeway_seconds:    0,
        }
    }

    /// @covers: verify
    #[test]
    fn test_verify_valid_hs256_token_returns_claims() {
        let secret = b"testsecret";
        let v = JwtVerifier::from_config(&config(secret)).unwrap();
        let token = hs256_token(secret, "alice", 3600);
        let claims = v.verify(&token).expect("valid token");
        assert_eq!(claims.sub.as_deref(), Some("alice"));
    }

    /// @covers: verify
    #[test]
    fn test_verify_wrong_secret_returns_invalid() {
        let v = JwtVerifier::from_config(&config(b"correct")).unwrap();
        let token = hs256_token(b"wrong", "bob", 3600);
        assert!(matches!(v.verify(&token), Err(VerifierError::Invalid(_))));
    }

    /// @covers: verify
    #[test]
    fn test_verify_expired_token_returns_expired_error() {
        let secret = b"key";
        let v = JwtVerifier::from_config(&config(secret)).unwrap();
        let token = hs256_token(secret, "x", -1);
        assert!(matches!(v.verify(&token), Err(VerifierError::Expired)));
    }

    /// @covers: verify
    #[test]
    fn test_verify_malformed_token_returns_invalid() {
        let v = JwtVerifier::from_config(&config(b"key")).unwrap();
        assert!(matches!(v.verify("not.a.jwt"), Err(VerifierError::Invalid(_))));
    }
}
