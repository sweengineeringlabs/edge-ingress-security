//! `HttpConfigValidator` — validates [`HttpConfig`] instances.

use crate::api::traits::Validator;
use crate::api::vo::http_config::HttpConfig;

/// Validates an [`HttpConfig`] by checking that required numeric fields
/// are nonzero and other constraints are met.
pub(crate) struct HttpConfigValidator<'a> {
    config: &'a HttpConfig,
}

impl<'a> HttpConfigValidator<'a> {
    /// Wrap a reference to a config for validation.
    pub(crate) fn new(config: &'a HttpConfig) -> Self {
        Self { config }
    }
}

impl<'a> Validator for HttpConfigValidator<'a> {
    /// Returns `Ok(())` when the config is valid.
    fn validate(&self) -> Result<(), String> {
        if self.config.timeout_secs == 0 {
            return Err("timeout_secs must be greater than zero".to_string());
        }
        if self.config.connect_timeout_secs == 0 {
            return Err("connect_timeout_secs must be greater than zero".to_string());
        }
        Ok(())
    }
}

/// Implement `Validator` directly on `HttpConfig` for ergonomic use via SAF.
impl Validator for HttpConfig {
    fn validate(&self) -> Result<(), String> {
        HttpConfigValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wraps_config_reference() {
        let cfg = HttpConfig::default();
        let v = HttpConfigValidator::new(&cfg);
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_validate_returns_ok_for_valid_config() {
        let cfg = HttpConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_returns_err_when_timeout_secs_is_zero() {
        let cfg = HttpConfig {
            timeout_secs: 0,
            ..Default::default()
        };
        let err = cfg.validate().unwrap_err();
        assert!(err.contains("timeout_secs"), "{err}");
    }

    #[test]
    fn test_validate_returns_err_when_connect_timeout_secs_is_zero() {
        let cfg = HttpConfig {
            connect_timeout_secs: 0,
            ..Default::default()
        };
        let err = cfg.validate().unwrap_err();
        assert!(err.contains("connect_timeout_secs"), "{err}");
    }
}
