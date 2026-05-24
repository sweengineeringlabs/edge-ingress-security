//! gRPC inbound interceptor trait.

pub(crate) mod authorization_interceptor;
pub(crate) mod grpc;

pub use crate::api::types::interceptor::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
pub use authorization_interceptor::AuthorizationInterceptor;
pub use grpc::{GrpcIngressInterceptor, GrpcIngressInterceptorChain};
