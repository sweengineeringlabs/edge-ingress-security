//! gRPC server trait and configuration error types.

pub(crate) mod grpc_server;
pub(crate) mod grpc_server_config_error;

pub use grpc_server::GrpcServer;
pub use grpc_server_config_error::GrpcServerConfigError;
