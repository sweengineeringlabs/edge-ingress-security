//! Integration tests for SEA trait contracts in `api/traits.rs`.

use swe_edge_ingress_grpc_auth_mtls::{is_processor, MtlsAuthInterceptor};

/// @covers: is_processor
#[test]
fn test_processor_trait_is_satisfied_by_mtls_auth_interceptor() {
    let interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
    assert!(
        is_processor(&interceptor),
        "MtlsAuthInterceptor must satisfy the Processor trait"
    );
}
