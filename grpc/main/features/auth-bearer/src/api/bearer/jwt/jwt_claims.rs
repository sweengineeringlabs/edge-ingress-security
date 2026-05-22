//! Interface counterpart for [`crate::core::bearer::jwt::claims`].
//!
//! Declares the shape of the JWT claims value object.  The concrete
//! implementation lives in `core/bearer/jwt/claims.rs` and is used
//! internally during token validation.  This type is not part of the
//! public consumer API — it is gated behind `pub(crate)` module paths.
//!
//! Use [`super::jwt_claims_builder::JwtClaimsBuilder`] to construct instances
//! of [`JwtClaims`] in tests and internal fixtures.

use serde::{Deserialize, Serialize};

/// Standard JWT claims set — interface declaration.
///
/// Used during bearer token validation to carry the verified identity.
/// Consumers never construct this directly; they receive the verified
/// `sub` claim via the interceptor's metadata output.
///
/// Use [`super::jwt_claims_builder::JwtClaimsBuilder`] to construct instances
/// in tests and internal fixtures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Token issuer (`iss` claim).
    pub iss: String,
    /// Token audience (`aud` claim).
    pub aud: String,
    /// Token subject (`sub` claim) — the verified identity.
    pub sub: String,
    /// Expiry as Unix timestamp.
    pub exp: u64,
    /// Issued-at as Unix timestamp.
    pub iat: u64,
}
