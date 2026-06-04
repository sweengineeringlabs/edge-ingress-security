//! Primary processing contract for mTLS auth interceptors.

/// Marks a type as an mTLS auth processor in the SEA pipeline.
pub trait Processor: Send + Sync {
    /// Self-describe the processor for logging and health checks.
    fn describe(&self) -> &'static str;
}
