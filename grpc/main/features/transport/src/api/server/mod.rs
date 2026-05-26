//! gRPC server port and concrete server declarations.
//!
//! Re-exports from [`crate::api::types::server`] and [`crate::api::error`].

pub(crate) mod grpc;
pub(crate) mod tonic_grpc_server;

pub use crate::api::error::TonicServerError;
pub use crate::api::types::server::{
    TonicGrpcServer, TonicGrpcServerBuilder, MAX_MESSAGE_BYTES,
    MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG,
};
pub use grpc::{GrpcServer, GrpcServerConfigError};
