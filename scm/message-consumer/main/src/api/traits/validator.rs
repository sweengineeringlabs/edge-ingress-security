//! SEA interface contract — Validator trait for consumer configuration.

/// Validates a consumer configuration value before use.
pub trait Validator {
    /// Returns `Ok(())` when valid, or a human-readable error string.
    fn validate(&self) -> Result<(), String>;
}
