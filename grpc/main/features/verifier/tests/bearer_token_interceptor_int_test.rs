//! Tests for `BearerTokenInterceptor`.
use swe_edge_ingress_grpc_verifier::BearerTokenInterceptor;

/// @covers: BearerTokenInterceptor
#[test]
fn verifier_struct_bearer_token_interceptor_is_exported_int_test() {
    let _ = std::any::type_name::<BearerTokenInterceptor>();
}
