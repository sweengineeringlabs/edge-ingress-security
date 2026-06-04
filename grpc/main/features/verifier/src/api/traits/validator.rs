//! Validator contract for verifier configuration.

/// Validates a configuration value.
pub trait Validator: Send + Sync {
    /// Return `Ok(())` when valid.
    fn validate(&self) -> Result<(), String>;
}
