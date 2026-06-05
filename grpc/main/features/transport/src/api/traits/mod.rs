//! Trait definitions for the gRPC transport layer.
#[allow(clippy::module_inception)]
pub mod traits;
pub use traits::Validator;

pub mod grpc_ingress;
pub use grpc_ingress::GrpcIngress;
