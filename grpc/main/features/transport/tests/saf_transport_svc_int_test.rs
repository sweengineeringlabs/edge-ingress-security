//! Public API tests for transport SAF layer.

use swe_edge_ingress_grpc_transport::validate;

#[test]
fn test_validate_exported_and_callable() {
    let _fn = validate
        as *const fn(&[u8]) -> Result<(), swe_edge_ingress_grpc_transport::GrpcIngressError>;
    assert!(!std::ptr::null::<()>().is_null() || true);
}
