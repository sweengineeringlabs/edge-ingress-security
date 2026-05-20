//! Interface contract for HTTP config validation.
//!
//! This file is the api/ counterpart to `core::validator::http_config_validator`
//! per SEA Rule 121.

/// Marker trait for HTTP config validators.
///
/// Types implementing this trait validate HTTP configuration values
/// following the `core::validator::HttpConfigValidatorPort` contract.
pub trait HttpConfigValidatorPort: Send + Sync {
    /// Returns `Ok(())` when the config is valid, or a human-readable error.
    fn validate_config(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValidConfig;
    impl HttpConfigValidatorPort for AlwaysValidConfig {
        fn validate_config(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_http_config_validator_is_object_safe() {
        fn _assert(_: &dyn HttpConfigValidatorPort) {}
    }

    #[test]
    fn test_http_config_validator_ok_returns_unit() {
        assert!(AlwaysValidConfig.validate_config().is_ok());
    }
}
