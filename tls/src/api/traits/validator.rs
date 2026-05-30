//! `Validator` — configuration validation contract.

/// Validates a value, returning a human-readable error on failure.
pub trait Validator: Send + Sync {
    /// Return `Err` with a description of the first validation failure,
    /// or `Ok(())` when the value is valid.
    fn validate(&self) -> Result<(), String>;
}
