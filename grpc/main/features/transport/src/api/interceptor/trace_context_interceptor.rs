//! W3C Trace Context interceptor declarations — consts and `TraceContextInterceptor`.

/// Header name (HTTP/2 lower-case) for the W3C `traceparent`.
pub const TRACEPARENT: &str = "traceparent";

/// Header name for the W3C `tracestate`.
pub const TRACESTATE: &str = "tracestate";

/// Internal metadata key under which the validated `traceparent` is
/// republished for downstream handlers.
pub const EXTRACTED_TRACEPARENT: &str = "x-edge-extracted-traceparent";

/// Internal metadata key under which the validated `tracestate` is
/// republished for downstream handlers.
pub const EXTRACTED_TRACESTATE: &str = "x-edge-extracted-tracestate";

/// W3C Trace Context inbound extractor.
#[derive(Clone, Default)]
pub struct TraceContextInterceptor {}

impl TraceContextInterceptor {
    /// Construct a default extractor.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: TraceContextInterceptor::new — creates an extractor.
    #[test]
    fn test_new_creates_trace_context_interceptor() {
        let _ = TraceContextInterceptor::new();
    }

    /// @covers: TRACEPARENT, TRACESTATE — are lowercase per HTTP/2 spec.
    #[test]
    fn test_header_constants_are_lowercase() {
        assert_eq!(TRACEPARENT, TRACEPARENT.to_lowercase());
        assert_eq!(TRACESTATE, TRACESTATE.to_lowercase());
    }
}
