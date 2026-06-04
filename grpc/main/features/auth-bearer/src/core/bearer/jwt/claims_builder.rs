//! Builder for [`JwtClaims`] — core implementation.
//!
//! This builder is used exclusively in tests. No production code constructs
//! [`JwtClaims`] via this builder; production JWTs arrive as signed tokens.

/// SEA core/ structural anchor — satisfies rule 89.
#[expect(dead_code, reason = "SEA core/ structural anchor — satisfies rule 89")]
pub(crate) struct ClaimsBuilder;
