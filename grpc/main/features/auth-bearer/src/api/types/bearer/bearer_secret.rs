//! Symmetric / asymmetric secret material for JWT verification.

use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

/// Symmetric / asymmetric secret material for JWT verification.
///
/// `Hs256` carries a raw byte secret; comparisons MUST go through
/// [`BearerSecret::ct_eq_hs256`] which uses `subtle::ConstantTimeEq`.
/// Asymmetric variants carry PEM-encoded key bytes.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_auth_bearer::BearerSecret;
///
/// let s1 = BearerSecret::Hs256 { secret: b"shared-secret".to_vec() };
/// let s2 = BearerSecret::Hs256 { secret: b"shared-secret".to_vec() };
/// let s3 = BearerSecret::Hs256 { secret: b"other-secret".to_vec() };
///
/// assert!(s1.ct_eq_hs256(&s2));
/// assert!(!s1.ct_eq_hs256(&s3));
/// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_auth_bearer::BearerSecret;
    ///
    /// let a = BearerSecret::Hs256 { secret: b"key".to_vec() };
    /// let b = BearerSecret::Hs256 { secret: b"key".to_vec() };
    /// let c = BearerSecret::Rs256 { public_pem: vec![] };
    ///
    /// assert!(a.ct_eq_hs256(&b));
    /// assert!(!a.ct_eq_hs256(&c)); // different variants → false
    /// ```
    pub fn ct_eq_hs256(&self, other: &Self) -> bool {
        match (self, other) {
            (BearerSecret::Hs256 { secret: a }, BearerSecret::Hs256 { secret: b }) => {
                a.as_slice().ct_eq(b.as_slice()).into()
            }
            _ => false,
        }
    }
}
