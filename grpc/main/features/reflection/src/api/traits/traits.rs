//! SEA interface contracts — core trait definitions for the gRPC reflection service.

/// Validation contract for inbound wire payloads before they reach the processor.
///
/// Implementors inspect the raw byte slice of an inbound frame and return
/// `Ok(())` when the payload is well-formed, or `Err(String)` with a
/// human-readable reason when it is not.
pub trait Validator: Send + Sync {
    /// Validate a raw inbound frame. Returns `Ok(())` when valid.
    fn validate(&self, raw: &[u8]) -> Result<(), String>;
}
