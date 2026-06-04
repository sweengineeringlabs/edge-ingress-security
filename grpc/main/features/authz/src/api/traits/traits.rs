//! SEA interface contracts — primary processing and validation traits.

/// Validates authz configuration before use.
pub trait Validator: Send + Sync {
    /// Returns `Ok(())` when the configuration is valid.
    fn validate(&self) -> Result<(), String>;
}
