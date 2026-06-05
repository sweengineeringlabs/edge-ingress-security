//! A runnable gRPC server trait.

use futures::future::BoxFuture;

use crate::api::error::GrpcIngressError;

/// A runnable gRPC server that drives a [`GrpcIngress`](super::super::port::grpc::GrpcIngress) handler.
pub trait GrpcServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), GrpcIngressError>>;
}
