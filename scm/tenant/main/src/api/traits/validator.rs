//! [`Validator`] — validation contract for tenant configuration types.

/// Validates a tenant configuration value.
pub trait Validator {
    /// Validate this value.
    ///
    /// Returns `Ok(())` if valid, or `Err(String)` with a human-readable reason.
    fn validate(&self) -> Result<(), String>;
}
