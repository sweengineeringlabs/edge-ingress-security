//! gRPC inbound interceptor trait.

pub(crate) mod authorization_interceptor;
pub(crate) mod grpc;
pub(crate) mod trace_context_interceptor;

pub use authorization_interceptor::AuthorizationInterceptor;
pub use grpc::{GrpcIngressInterceptor, GrpcIngressInterceptorChain};
