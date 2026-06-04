//! Primary processing contract for authz interceptors.

/// Marks a type as an authz processor in the SEA pipeline.
pub trait Processor: Send + Sync {
    /// Self-describe the processor for logging and health checks.
    fn describe(&self) -> &'static str;
}
