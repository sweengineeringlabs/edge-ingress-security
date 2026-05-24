//! Integration tests for the [`Validator`] trait contract.

use swe_edge_ingress_grpc_auth_mtls::{is_validator, MtlsAuthInterceptor};

/// @covers: is_validator
#[test]
fn test_mtls_auth_interceptor_implements_validator_trait() {
    let interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
    assert!(
        is_validator(&interceptor),
        "MtlsAuthInterceptor must satisfy the Validator trait"
    );
}
