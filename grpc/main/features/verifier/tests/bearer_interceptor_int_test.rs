//! Tests for BearerInterceptor API module.

use swe_edge_ingress_grpc_verifier::BearerTokenInterceptor;

/// @covers: BearerTokenInterceptor
#[test]
fn test_bearer_token_interceptor_struct_is_publicly_exported() {
    let _ = std::any::type_name::<BearerTokenInterceptor>();
}
