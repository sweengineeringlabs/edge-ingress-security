//! Trait definitions for the gRPC transport layer.
#[allow(clippy::module_inception)]
pub mod traits;
pub use traits::Validator;

pub mod grpc_ingress;
pub use grpc_ingress::GrpcIngress;

pub mod grpc_ingress_interceptor;
pub use grpc_ingress_interceptor::GrpcIngressInterceptor;

pub mod authorization_interceptor;
pub use authorization_interceptor::AuthorizationInterceptor;

pub mod audit_sink;
pub use audit_sink::AuditSink;

pub mod grpc_server;
pub use grpc_server::GrpcServer;
