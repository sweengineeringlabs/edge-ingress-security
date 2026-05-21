//! `validate` SAF facade function for the [`Validator`] trait.

use crate::api::traits::Validator;

/// Validate a value using its [`Validator`] implementation.
///
/// Returns `Ok(())` when the value is valid, or a human-readable error message.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::http::http_config::HttpConfig;

    /// @covers: validate
    #[test]
    fn test_validate_returns_ok_for_default_http_config() {
        let cfg = HttpConfig::default();
        assert!(validate(&cfg).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_returns_err_for_zero_timeout_config() {
        let cfg = HttpConfig {
            timeout_secs: 0,
            ..Default::default()
        };
        assert!(validate(&cfg).is_err());
    }
}
