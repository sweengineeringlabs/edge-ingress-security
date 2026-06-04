//! gRPC inbound interceptor types.

pub(crate) mod grpc;
pub(crate) mod trace_context_interceptor;

pub use grpc::{GrpcIngressInterceptor, GrpcIngressInterceptorChain};
pub use trace_context_interceptor::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
