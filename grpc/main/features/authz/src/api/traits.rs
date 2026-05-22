//! SEA interface contracts — primary processing and validation traits.

pub use crate::api::processor::Processor;

/// Validates authz configuration before use.
pub trait Validator: Send + Sync {
    /// Returns `Ok(())` when the configuration is valid.
    fn validate(&self) -> Result<(), String>;
}
