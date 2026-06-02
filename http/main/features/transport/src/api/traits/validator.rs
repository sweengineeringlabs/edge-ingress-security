//! SEA interface contract — inbound transport traits.

/// Validates an inbound configuration or request value.
///
/// All inbound port implementors must also implement this trait to satisfy
/// SEA rule 155 (every non-orchestrator crate must have a `Validator` in `api/traits.rs`).
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a human-readable error.
    fn validate(&self) -> Result<(), String>;
}
