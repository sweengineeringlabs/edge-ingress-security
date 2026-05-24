//! A runnable gRPC server trait.

use futures::future::BoxFuture;

use crate::api::port::grpc::GrpcIngressError;

/// A runnable gRPC server that drives a [`GrpcIngress`](super::super::port::grpc::GrpcIngress) handler.
pub trait GrpcServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), GrpcIngressError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_server_is_object_safe() {
        fn _assert(_: &dyn GrpcServer) {}
    }
}
