//! gRPC inbound interceptor types.

pub(crate) mod grpc_ingress_interceptor;

pub use crate::api::types::interceptor::GrpcIngressInterceptorChain;
pub use grpc_ingress_interceptor::GrpcIngressInterceptor;
