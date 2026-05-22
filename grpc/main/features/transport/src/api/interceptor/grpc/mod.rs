//! gRPC inbound interceptor types.

pub(crate) mod grpc_ingress_interceptor;
pub(crate) mod grpc_ingress_interceptor_chain;

pub use grpc_ingress_interceptor::GrpcIngressInterceptor;
pub use grpc_ingress_interceptor_chain::GrpcIngressInterceptorChain;
