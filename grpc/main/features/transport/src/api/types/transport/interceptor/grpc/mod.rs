//! gRPC-specific interceptor types.
pub mod grpc_ingress_interceptor;
pub mod grpc_ingress_interceptor_chain;
pub use grpc_ingress_interceptor_chain::GrpcIngressInterceptorChain;
