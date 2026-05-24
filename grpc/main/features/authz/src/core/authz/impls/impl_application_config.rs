//! [`Validator`] implementation for [`ApplicationConfig`].

use crate::api::application::ApplicationConfig;
use crate::api::traits::Validator;

impl Validator for ApplicationConfig {
    fn validate(&self) -> Result<(), String> {
        match self.default_policy.as_str() {
            "allow" | "deny" => Ok(()),
            other => Err(format!(
                "invalid default_policy '{}': must be 'allow' or 'deny'",
                other
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: validate
    #[test]
    fn test_validate_rejects_unknown_policy_value() {
        let cfg = ApplicationConfig {
            default_policy: "unknown".into(),
        };
        assert!(cfg.validate().is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_accepts_deny_and_allow() {
        let deny = ApplicationConfig {
            default_policy: "deny".into(),
        };
        let allow = ApplicationConfig {
            default_policy: "allow".into(),
        };
        assert!(deny.validate().is_ok());
        assert!(allow.validate().is_ok());
    }
}
