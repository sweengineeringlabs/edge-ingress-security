//! Integration tests for GrpcIngressResult.

use swe_edge_ingress_grpc_transport::{GrpcIngressError, GrpcIngressResult};

/// @covers: GrpcIngressResult
#[test]
fn test_grpc_ingress_result_is_result_alias() {
    let ok: GrpcIngressResult<u32> = Ok(42);
    assert!(matches!(ok, Ok(42)));
    let err: GrpcIngressResult<u32> = Err(GrpcIngressError::Internal("fail".into()));
    assert!(err.is_err());
}
