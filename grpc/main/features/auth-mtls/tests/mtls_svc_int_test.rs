//! Integration tests for SAF service functions.

use swe_edge_ingress_grpc_auth_mtls::{
    is_authorization_interceptor, is_processor, MtlsAuthInterceptor,
};

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
