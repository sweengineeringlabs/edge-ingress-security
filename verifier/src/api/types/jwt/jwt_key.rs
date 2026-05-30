//! `JwtKey` — key material for JWT signature verification.

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
