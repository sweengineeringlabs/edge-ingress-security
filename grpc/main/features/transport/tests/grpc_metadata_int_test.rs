//! Integration tests for GrpcMetadata.

use swe_edge_ingress_grpc_transport::GrpcMetadata;

/// @covers: GrpcMetadata::default
#[test]
fn test_grpc_metadata_default_has_empty_headers() {
    let m = GrpcMetadata::default();
    assert!(m.headers.is_empty());
}
