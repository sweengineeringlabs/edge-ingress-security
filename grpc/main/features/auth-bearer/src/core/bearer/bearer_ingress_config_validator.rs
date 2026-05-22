//! Core implementation of [`Validator`] for [`BearerIngressConfig`].

use crate::api::traits::Validator;
use crate::api::BearerIngressConfig;

impl Validator for BearerIngressConfig {
    /// Validate that issuer and audience are non-empty.
    fn validate(&self) -> Result<(), String> {
        if self.expected_issuer.is_empty() {
            return Err("expected_issuer must not be empty".into());
        }
        if self.expected_audience.is_empty() {
            return Err("expected_audience must not be empty".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::api::bearer::bearer_secret::BearerSecret;
    use crate::api::traits::Validator;
    use crate::api::BearerIngressConfig;

    fn make_config(issuer: &str, audience: &str) -> BearerIngressConfig {
        BearerIngressConfig {
            secret: BearerSecret::Hs256 {
                secret: b"sec".to_vec(),
            },
            expected_issuer: issuer.into(),
            expected_audience: audience.into(),
            leeway_seconds: 0,
        }
    }

    #[test]
    fn test_validate_valid_config_returns_ok() {
        let cfg = make_config("svc-a", "svc-b");
        assert!(
            cfg.validate().is_ok(),
            "config with non-empty issuer and audience must be valid"
        );
    }

    #[test]
    fn test_validate_empty_issuer_returns_err() {
        let cfg = make_config("", "svc-b");
        let err = cfg
            .validate()
            .expect_err("empty issuer must fail validation");
        assert!(
            err.contains("expected_issuer"),
            "error must reference the failing field"
        );
    }

    #[test]
    fn test_validate_empty_audience_returns_err() {
        let cfg = make_config("svc-a", "");
        let err = cfg
            .validate()
            .expect_err("empty audience must fail validation");
        assert!(
            err.contains("expected_audience"),
            "error must reference the failing field"
        );
    }
}
