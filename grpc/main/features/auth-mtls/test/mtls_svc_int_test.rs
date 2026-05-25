//! Integration tests for SAF service functions.

use swe_edge_ingress_grpc_auth_mtls::{
    create_config_builder, is_authorization_interceptor, is_processor, is_validator,
    MtlsAuthInterceptor,
};

/// @covers: create_config_builder — returns builder with name and version
#[test]
fn test_create_config_builder_returns_builder_with_name_and_version() {
    let builder = create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: is_authorization_interceptor
#[test]
fn test_is_authorization_interceptor_returns_true() {
    assert!(
        is_authorization_interceptor(),
        "mTLS interceptor must be an authorization gate"
    );
}

/// @covers: is_processor
#[test]
fn test_is_processor_returns_true_for_mtls_interceptor() {
    let interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
    assert!(is_processor(&interceptor));
}

/// @covers: is_validator
#[test]
fn test_is_validator_returns_true_for_mtls_interceptor() {
    let interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
    assert!(is_validator(&interceptor));
}
