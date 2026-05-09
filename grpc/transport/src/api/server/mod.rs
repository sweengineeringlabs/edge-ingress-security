//! gRPC server port and concrete server declarations.

pub mod tonic_grpc_server;

use futures::future::BoxFuture;

use crate::api::port::grpc_inbound::GrpcInboundError;

/// A runnable gRPC server that drives a [`GrpcInbound`](super::port::grpc_inbound::GrpcInbound) handler.
pub trait GrpcServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), GrpcInboundError>>;
}
