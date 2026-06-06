//! `Processor` — primary trait for the verifier crate.
/// Identifies the primary processing role of this crate.
pub trait Processor: Send + Sync {
    /// Describe this processor.
    fn describe(&self) -> &'static str;
}
