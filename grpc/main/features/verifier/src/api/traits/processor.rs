//! Primary processing contract for the verifier.

/// Marks a type as a verifier-layer processor.
pub trait Processor: Send + Sync {
    /// Describe this processor.
    fn describe(&self) -> &'static str;
}
