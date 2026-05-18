//! Configuration for the inbound bearer interceptor.

use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

/// Symmetric / asymmetric secret material for JWT verification.
///
/// `Hs256` carries a raw byte secret; comparisons MUST go through
/// [`BearerSecret::ct_eq_hs256`] which uses `subtle::ConstantTimeEq`.
/// Asymmetric variants carry PEM-encoded key bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BearerSecret {
    /// HS256 — shared symmetric secret.
    Hs256 {
        /// Raw secret bytes (UTF-8 in TOML; arbitrary bytes via API).
        secret: Vec<u8>,
    },
    /// RS256 — public PEM bytes for signature verification.
    Rs256 {
        /// PEM-encoded public key.
        #[serde(default)]
        public_pem: Vec<u8>,
    },
}

impl BearerSecret {
    /// Constant-time equality on HS256 secrets.  Returns `false` for
    /// different variants — algorithm-mismatch is never "equal".
    pub fn ct_eq_hs256(&self, other: &Self) -> bool {
        match (self, other) {
            (BearerSecret::Hs256 { secret: a }, BearerSecret::Hs256 { secret: b }) => {
                a.as_slice().ct_eq(b.as_slice()).into()
            }
            _ => false,
        }
    }
}

/// Inbound (server) bearer config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearerInboundConfig {
    /// Verification key material.
    pub secret: BearerSecret,
    /// Required `iss` value — tokens with a different issuer are rejected.
    pub expected_issuer: String,
    /// Required `aud` value — tokens with a different audience are rejected.
    pub expected_audience: String,
    /// Maximum acceptable clock skew when checking `exp`/`nbf`, in seconds.
    pub leeway_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: BearerSecret::ct_eq_hs256 — equal secrets compare equal.
    #[test]
    fn test_ct_eq_hs256_returns_true_for_identical_secrets() {
        let a = BearerSecret::Hs256 {
            secret: b"secret".to_vec(),
        };
        let b = BearerSecret::Hs256 {
            secret: b"secret".to_vec(),
        };
        assert!(a.ct_eq_hs256(&b));
    }

    /// @covers: BearerSecret::ct_eq_hs256 — different secrets compare unequal.
    #[test]
    fn test_ct_eq_hs256_returns_false_for_different_secrets() {
        let a = BearerSecret::Hs256 {
            secret: b"alpha".to_vec(),
        };
        let b = BearerSecret::Hs256 {
            secret: b"beta".to_vec(),
        };
        assert!(!a.ct_eq_hs256(&b));
    }

    /// @covers: BearerSecret::ct_eq_hs256 — variant mismatch is never equal.
    #[test]
    fn test_ct_eq_hs256_returns_false_for_variant_mismatch() {
        let a = BearerSecret::Hs256 {
            secret: b"x".to_vec(),
        };
        let b = BearerSecret::Rs256 { public_pem: vec![] };
        assert!(!a.ct_eq_hs256(&b));
    }
}
