//! Integration tests for GrpcIngressError.

use swe_edge_ingress_grpc_transport::{GrpcIngressError, GrpcStatusCode};

/// @covers: GrpcIngressError::Internal
#[test]
fn test_grpc_ingress_error_internal_formats_correctly() {
    let err = GrpcIngressError::Internal("fail".into());
    assert!(err.to_string().contains("fail"));
}

/// @covers: GrpcIngressError::Status
#[test]
fn test_grpc_ingress_error_status_variant_carries_code_and_message() {
    let err = GrpcIngressError::Status(GrpcStatusCode::Aborted, "tx aborted".into());
    let s = err.to_string();
    assert!(s.contains("Aborted"));
    assert!(s.contains("tx aborted"));
}
