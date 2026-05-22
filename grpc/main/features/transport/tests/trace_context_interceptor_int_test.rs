//! Integration tests for TraceContextInterceptor.

use swe_edge_ingress_grpc_transport::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};

/// @covers: TraceContextInterceptor::new
#[test]
fn test_new_creates_trace_context_interceptor() {
    let _ = TraceContextInterceptor::new();
}

/// @covers: TRACEPARENT
/// @covers: TRACESTATE
#[test]
fn test_header_constants_are_lowercase() {
    assert_eq!(TRACEPARENT, TRACEPARENT.to_lowercase());
    assert_eq!(TRACESTATE, TRACESTATE.to_lowercase());
    assert_eq!(EXTRACTED_TRACEPARENT, EXTRACTED_TRACEPARENT.to_lowercase());
    assert_eq!(EXTRACTED_TRACESTATE, EXTRACTED_TRACESTATE.to_lowercase());
}
