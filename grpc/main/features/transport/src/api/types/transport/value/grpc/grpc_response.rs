//! gRPC response envelope.

use super::grpc_metadata::GrpcMetadata;

/// A gRPC response envelope.
#[derive(Debug, Clone)]
pub struct GrpcResponse {
    /// Raw protobuf-encoded response bytes.
    pub body: Vec<u8>,
    /// Response metadata (trailers).
    pub metadata: GrpcMetadata,
}
