//! Error types.

pub(crate) mod tonic_server_error;

pub use tonic_server_error::TonicServerError;

pub mod grpc_ingress_error;
pub use grpc_ingress_error::GrpcIngressError;
