//! Integration tests for the SAF validate function.

use swe_edge_ingress_grpc_transport::{validate, GrpcServerConfig};

/// @covers: validate
#[test]
fn test_validate_returns_ok_for_plaintext_config() {
    let cfg = GrpcServerConfig::default().allow_plaintext();
    assert!(validate(&cfg).is_ok());
}

/// @covers: validate
#[test]
fn test_validate_returns_err_for_tls_required_without_tls_config() {
    let cfg = GrpcServerConfig::default();
    assert!(validate(&cfg).is_err());
}
