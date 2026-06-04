//! Integration tests for GrpcServerConfigError.

use swe_edge_ingress_grpc_transport::GrpcServerConfigError;

/// @covers: GrpcServerConfigError::TlsRequiredButMissing
#[test]
fn test_grpc_server_config_error_tls_required_has_descriptive_message() {
    let e = GrpcServerConfigError::TlsRequiredButMissing;
    assert!(e.to_string().contains("tls_required"));
}
