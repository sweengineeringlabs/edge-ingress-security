//! Integration tests for the [`Validator`] trait and its implementations.
//!
//! Rule 105: each api/ trait has a corresponding test file in tests/.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_grpc_transport::{validate, GrpcServerConfig};

/// GrpcServerConfig validates successfully when TLS is not required.
#[test]
fn test_grpc_server_config_validates_when_tls_not_required() {
    let cfg = GrpcServerConfig::default().allow_plaintext();
    assert!(validate(&cfg).is_ok(), "plaintext config must be valid");
}

/// GrpcServerConfig fails validation when tls_required but no TLS config is attached.
#[test]
fn test_grpc_server_config_fails_validation_when_tls_required_without_tls_config() {
    let cfg = GrpcServerConfig::default(); // tls_required = true by default
    let result = validate(&cfg);
    assert!(result.is_err(), "tls_required without tls config must fail");
    assert!(
        result.unwrap_err().contains("tls_required"),
        "error must mention tls_required"
    );
}

/// validate() SAF function delegates correctly to the Validator impl.
#[test]
fn test_validate_saf_function_returns_ok_for_valid_config() {
    let cfg = GrpcServerConfig::default().allow_plaintext();
    assert!(validate(&cfg).is_ok());
}

/// validate() works for any type that implements Validator.
#[test]
fn test_validate_saf_function_returns_err_for_invalid_config() {
    let cfg = GrpcServerConfig::default(); // tls_required = true, no tls config
    assert!(validate(&cfg).is_err());
}
