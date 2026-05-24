//! Integration tests for GrpcResponse.

use swe_edge_ingress_grpc_transport::{GrpcMetadata, GrpcResponse};

/// @covers: GrpcResponse
#[test]
fn test_grpc_response_holds_body_bytes() {
    let resp = GrpcResponse {
        body: vec![0x08, 0x01],
        metadata: GrpcMetadata::default(),
    };
    assert_eq!(resp.body, vec![0x08, 0x01]);
}
