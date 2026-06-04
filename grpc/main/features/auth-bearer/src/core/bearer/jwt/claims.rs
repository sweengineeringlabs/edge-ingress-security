//! Implementation counterpart for [`crate::api::bearer::jwt::jwt_claims`].
//!
//! Re-exports [`JwtClaims`] from the api layer and adds the constructor
//! implementation used internally by the interceptor.

pub(crate) use crate::api::bearer::jwt::jwt_claims::JwtClaims;

impl JwtClaims {
    /// Construct a new [`JwtClaims`] value.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "SEA core/ builder — used by tests")
    )]
    pub(crate) fn new(
        iss: impl Into<String>,
        sub: impl Into<String>,
        aud: impl Into<String>,
        exp: u64,
        iat: u64,
    ) -> Self {
        Self {
            iss: iss.into(),
            sub: sub.into(),
            aud: aud.into(),
            exp,
            iat,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JwtClaims;

    #[test]
    fn test_new_populates_all_fields_correctly() {
        let claims = JwtClaims::new("issuer", "subject", "audience", 9999, 1000);
        assert_eq!(claims.iss, "issuer", "iss must match constructor arg");
        assert_eq!(claims.sub, "subject", "sub must match constructor arg");
        assert_eq!(claims.aud, "audience", "aud must match constructor arg");
        assert_eq!(claims.exp, 9999, "exp must match constructor arg");
        assert_eq!(claims.iat, 1000, "iat must match constructor arg");
    }

    #[test]
    fn test_new_with_empty_strings_produces_empty_fields() {
        let claims = JwtClaims::new("", "", "", 0, 0);
        assert!(
            claims.iss.is_empty(),
            "iss must be empty when given empty string"
        );
        assert!(
            claims.sub.is_empty(),
            "sub must be empty when given empty string"
        );
        assert!(
            claims.aud.is_empty(),
            "aud must be empty when given empty string"
        );
    }
}
