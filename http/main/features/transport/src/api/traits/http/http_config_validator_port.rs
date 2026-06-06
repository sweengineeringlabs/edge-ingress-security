//! Interface contract for HTTP config validation.
//!
//! This file is the api/ counterpart to `core::validator::http_config_validator`
//! per SEA Rule 121.

/// Marker trait for HTTP config validators.
///
/// Types implementing this trait validate HTTP configuration values
/// following the `core::validator::HttpConfigValidatorPort` contract.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub trait HttpConfigValidatorPort: Send + Sync {
    /// Returns `Ok(())` when the config is valid, or a human-readable error.
    fn validate_config(&self) -> Result<(), String>;
}
