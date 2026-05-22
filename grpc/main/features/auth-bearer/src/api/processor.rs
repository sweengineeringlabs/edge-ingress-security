//! Primary processing contract for bearer auth interceptors.

/// Primary processing contract for bearer auth interceptors.
///
/// Implemented by [`crate::BearerIngressInterceptor`] to mark the type as a
/// first-class processor in the SEA pipeline.
#[allow(dead_code)]
pub trait Processor: Send + Sync {}
