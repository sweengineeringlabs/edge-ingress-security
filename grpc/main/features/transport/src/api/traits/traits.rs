//! SEA interface contract — inbound gRPC transport traits.

/// Validates an inbound configuration or interceptor value.
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a human-readable error.
    fn validate(&self) -> Result<(), String>;
}
