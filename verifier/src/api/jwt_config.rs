//! `JwtConfig` — configuration for `JwtVerifier`.

use serde::{Deserialize, Serialize};

/// Key material used to verify JWT signatures.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "algorithm", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JwtKey {
    /// HS256 — shared symmetric secret bytes.
    Hs256 {
        /// Raw HMAC secret bytes.
        secret: Vec<u8>,
    },
    /// RS256 — PEM-encoded RSA public key.
    Rs256 {
        /// PEM-encoded RSA public key.
        public_pem: String,
    },
    /// ES256 — PEM-encoded EC public key.
    Es256 {
        /// PEM-encoded EC public key.
        public_pem: String,
    },
}

/// Configuration for [`JwtVerifier`](crate::saf::JwtVerifier).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Key material and algorithm.
    pub key: JwtKey,
    /// Required `iss` claim value.  `None` skips issuer validation.
    #[serde(default)]
    pub required_issuer: Option<String>,
    /// Required `aud` claim value.  `None` skips audience validation.
    #[serde(default)]
    pub required_audience: Option<String>,
    /// Maximum clock skew in seconds when validating `exp`/`nbf`.
    #[serde(default = "default_leeway")]
    pub leeway_seconds: u64,
}

fn default_leeway() -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: JwtConfig — deserializes from TOML-shaped JSON.
    #[test]
    fn test_jwt_config_deserializes_hs256() {
        let json = r#"{"key":{"algorithm":"HS256","secret":[107,101,121]}}"#;
        let cfg: JwtConfig = serde_json::from_str(json).expect("deserialize");
        assert!(matches!(cfg.key, JwtKey::Hs256 { .. }));
        assert_eq!(cfg.leeway_seconds, 0);
    }

    /// @covers: JwtConfig — optional fields default to None / zero.
    #[test]
    fn test_jwt_config_optional_fields_default_correctly() {
        let json = r#"{"key":{"algorithm":"HS256","secret":[]}}"#;
        let cfg: JwtConfig = serde_json::from_str(json).unwrap();
        assert!(cfg.required_issuer.is_none());
        assert!(cfg.required_audience.is_none());
    }
}
