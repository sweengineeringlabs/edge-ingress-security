//! SEA interface contracts — re-exports primary traits.
pub use crate::api::processor::Processor;

/// Validation contract for mTLS auth interceptors.
pub trait Validator: Send + Sync {
    /// Validate the receiver's invariants.
    ///
    /// Returns `Ok(())` when all invariants hold, or an `Err` describing the
    /// first violation found.
    fn validate(&self) -> Result<(), String>;
}
