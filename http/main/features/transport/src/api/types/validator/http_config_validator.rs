//! Interface contract for HTTP config validation.
//!
//! This is the api/ counterpart to `core::validator::http_config_validator`
//! per SEA Rule 121.

use crate::api::traits::Validator;

/// Marker type indicating the HTTP config validation contract.
///
/// Any type that implements [`Validator`] can serve as an HTTP config validator.
/// This type is used to express the validation constraint on configuration structs.
pub struct HttpConfigValidator;

impl Validator for HttpConfigValidator {
    /// Always returns `Ok(())` — use `HttpConfig::validate()` for real validation.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_validator_is_a_marker_type() {
        let v = HttpConfigValidator;
        assert!(v.validate().is_ok());
    }
}
