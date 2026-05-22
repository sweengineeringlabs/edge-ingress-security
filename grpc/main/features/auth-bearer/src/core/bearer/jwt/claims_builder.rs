//! Builder for [`JwtClaims`] — core implementation.

use crate::api::bearer::jwt::claims_builder::ClaimsBuilder;
use crate::api::bearer::jwt::jwt_claims::JwtClaims;
use crate::api::bearer::jwt::jwt_claims_builder_contract::JwtClaimsBuilderContract;

/// Fluent builder for [`JwtClaims`].
#[allow(dead_code)]
pub(crate) struct JwtClaimsBuilder {
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[allow(dead_code)]
impl JwtClaimsBuilder {
    /// Start a new builder.
    pub(crate) fn new() -> Self {
        Self {
            iss: String::new(),
            sub: String::new(),
            aud: String::new(),
            exp: 0,
            iat: 0,
        }
    }

    /// Set the issuer (`iss`) claim.
    pub(crate) fn iss(mut self, iss: impl Into<String>) -> Self {
        self.iss = iss.into();
        self
    }

    /// Set the subject (`sub`) claim.
    pub(crate) fn sub(mut self, sub: impl Into<String>) -> Self {
        self.sub = sub.into();
        self
    }

    /// Set the audience (`aud`) claim.
    pub(crate) fn aud(mut self, aud: impl Into<String>) -> Self {
        self.aud = aud.into();
        self
    }

    /// Set the expiry (`exp`) as a Unix timestamp.
    pub(crate) fn exp(mut self, exp: u64) -> Self {
        self.exp = exp;
        self
    }

    /// Set the issued-at (`iat`) as a Unix timestamp.
    pub(crate) fn iat(mut self, iat: u64) -> Self {
        self.iat = iat;
        self
    }

    /// Build the [`JwtClaims`].
    pub(crate) fn build(self) -> JwtClaims {
        JwtClaims {
            iss: self.iss,
            sub: self.sub,
            aud: self.aud,
            exp: self.exp,
            iat: self.iat,
        }
    }
}

impl JwtClaimsBuilderContract for JwtClaimsBuilder {
    fn build(self) -> JwtClaims {
        JwtClaims {
            iss: self.iss,
            sub: self.sub,
            aud: self.aud,
            exp: self.exp,
            iat: self.iat,
        }
    }
}

impl ClaimsBuilder for JwtClaimsBuilder {
    fn build(self) -> JwtClaims {
        JwtClaims {
            iss: self.iss,
            sub: self.sub,
            aud: self.aud,
            exp: self.exp,
            iat: self.iat,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JwtClaimsBuilder;

    #[test]
    fn test_new_produces_zero_value_builder() {
        let b = JwtClaimsBuilder::new();
        let claims = b.build();
        assert!(claims.iss.is_empty(), "default iss must be empty");
        assert!(claims.sub.is_empty(), "default sub must be empty");
        assert!(claims.aud.is_empty(), "default aud must be empty");
        assert_eq!(claims.exp, 0, "default exp must be 0");
        assert_eq!(claims.iat, 0, "default iat must be 0");
    }

    #[test]
    fn test_iss_sets_issuer_field() {
        let claims = JwtClaimsBuilder::new().iss("my-issuer").build();
        assert_eq!(claims.iss, "my-issuer");
    }

    #[test]
    fn test_sub_sets_subject_field() {
        let claims = JwtClaimsBuilder::new().sub("user-42").build();
        assert_eq!(claims.sub, "user-42");
    }

    #[test]
    fn test_aud_sets_audience_field() {
        let claims = JwtClaimsBuilder::new().aud("api-gateway").build();
        assert_eq!(claims.aud, "api-gateway");
    }

    #[test]
    fn test_exp_sets_expiry_field() {
        let claims = JwtClaimsBuilder::new().exp(9_999_999).build();
        assert_eq!(claims.exp, 9_999_999);
    }

    #[test]
    fn test_iat_sets_issued_at_field() {
        let claims = JwtClaimsBuilder::new().iat(1_000_000).build();
        assert_eq!(claims.iat, 1_000_000);
    }

    #[test]
    fn test_build_produces_claims_with_all_fields_set() {
        let claims = JwtClaimsBuilder::new()
            .iss("svc-a")
            .sub("alice")
            .aud("svc-b")
            .exp(2_000_000)
            .iat(1_000_000)
            .build();
        assert_eq!(claims.iss, "svc-a");
        assert_eq!(claims.sub, "alice");
        assert_eq!(claims.aud, "svc-b");
        assert_eq!(claims.exp, 2_000_000);
        assert_eq!(claims.iat, 1_000_000);
    }
}
