//! Message stream type alias for gRPC inbound operations.

use super::grpc_ingress_result::GrpcIngressResult;

/// A stream of raw gRPC message bytes — one item per decoded gRPC frame.
pub type GrpcMessageStream =
    std::pin::Pin<Box<dyn futures::Stream<Item = GrpcIngressResult<Vec<u8>>> + Send>>;
