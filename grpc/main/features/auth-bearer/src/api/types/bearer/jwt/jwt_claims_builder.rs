//! Fluent builder for [`JwtClaims`] — api-layer declaration.
//!
//! Provides the builder interface for the [`JwtClaims`] value object.
//! The full implementation with all setter methods lives in
//! `core/bearer/jwt/claims_builder.rs`.

use super::jwt_claims::JwtClaims;

/// Fluent builder for [`JwtClaims`].
///
/// Use this builder to construct [`JwtClaims`] instances in tests and
/// internal fixtures rather than constructing the struct directly.

pub struct JwtClaimsBuilder {
    /// Issuer claim.
    pub iss: String,
    /// Audience claim.
    pub aud: String,
    /// Subject claim.
    pub sub: String,
    /// Expiry timestamp.
    pub exp: u64,
    /// Issued-at timestamp.
    pub iat: u64,
}


impl JwtClaimsBuilder {
    /// Build the [`JwtClaims`].
    pub fn build(self) -> JwtClaims {
        JwtClaims {
            iss: self.iss,
            aud: self.aud,
            sub: self.sub,
            exp: self.exp,
            iat: self.iat,
        }
    }
}
