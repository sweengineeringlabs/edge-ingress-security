//! Primary processing contract for bearer auth interceptors.

/// Primary processing contract for bearer auth interceptors.
///
/// Implemented by [`crate::BearerIngressInterceptor`] to mark the type as a
/// first-class processor in the SEA pipeline.
pub trait Processor: Send + Sync {
    /// Describe this processor (used to identify it in logs and health checks).
    fn describe(&self) -> &'static str;
}
