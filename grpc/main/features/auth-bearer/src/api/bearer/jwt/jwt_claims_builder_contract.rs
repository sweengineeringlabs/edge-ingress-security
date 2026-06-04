//! Interface counterpart for [`crate::core::bearer::jwt::claims_builder`].
//!
//! Declares the `JwtClaimsBuilderContract` trait that the core builder
//! satisfies.  Consumers do not use this type directly — fixture
//! construction goes through the concrete builder in `core/bearer/jwt/`.

use super::jwt_claims::JwtClaims;

/// Contract for fluent JWT claims builders.
///
/// Implemented by [`crate::core::bearer::jwt::claims_builder::JwtClaimsBuilder`].
#[expect(dead_code, reason = "SEA api/ interface anchor")]
pub trait JwtClaimsBuilderContract {
    /// Build the final [`JwtClaims`].
    fn build(self) -> JwtClaims;
}
