//! A runnable gRPC server trait.

use futures::future::BoxFuture;

use crate::api::port::grpc_inbound::GrpcInboundError;

/// A runnable gRPC server that drives a [`GrpcInbound`](super::super::port::grpc_inbound::GrpcInbound) handler.
pub trait GrpcServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), GrpcInboundError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_server_is_object_safe() {
        fn _assert(_: &dyn GrpcServer) {}
    }
}
