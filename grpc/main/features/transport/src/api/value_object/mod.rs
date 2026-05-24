//! gRPC value objects.
//!
//! Re-exports from [`crate::api::types::value_object`].

pub use crate::api::types::value_object::{
    is_reserved_peer_key, CompressionMode, GrpcMetadata, GrpcRequest, GrpcRequestBuilder,
    GrpcResponse, GrpcServerConfig, GrpcServerConfigBuilder, GrpcStatusCode, PeerIdentity,
    DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES, PEER_CERT_FINGERPRINT_SHA256,
    PEER_CN, PEER_IDENTITY, PEER_SAN_DNS, PEER_SAN_URI, RESERVED_PEER_PREFIXES,
};
