//! Integration tests for [`Processor`].

use swe_edge_ingress_grpc_auth_mtls::{is_processor, MtlsAuthInterceptor};

/// @covers: is_processor
#[test]
fn test_mtls_auth_interceptor_implements_processor_trait() {
    let interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
    assert!(is_processor(&interceptor));
}
