//! `JwtKey` — key material for JWT signature verification.

use serde::{Deserialize, Serialize};

/// Key material used to verify JWT signatures.
///
/// Passed as the `key` field of [`JwtConfig`](crate::JwtConfig).
/// Choose the variant that matches the algorithm your identity provider uses:
///
/// | Variant | Algorithm | Key format |
/// |---------|-----------|------------|
/// | `Hs256` | HS256 | shared secret bytes |
/// | `Rs256` | RS256 | PEM RSA public key |
/// | `Es256` | ES256 | PEM EC public key |
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_verifier::JwtKey;
///
/// // HS256 — symmetric shared secret.
/// let key = JwtKey::Hs256 { secret: b"super-secret".to_vec() };
///
/// // RS256 — asymmetric RSA public key.
/// let key = JwtKey::Rs256 { public_pem: "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----".to_string() };
///
/// // ES256 — asymmetric EC public key.
/// let key = JwtKey::Es256 { public_pem: "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----".to_string() };
/// ```
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
