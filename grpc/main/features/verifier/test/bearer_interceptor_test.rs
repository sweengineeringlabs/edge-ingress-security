//! Tests for BearerInterceptor API module.

use swe_edge_ingress_grpc_verifier::BearerInterceptor;

#[test]
fn test_bearer_interceptor_trait_is_available() {
    // Verify BearerInterceptor trait is accessible from api layer.
    fn _assert_trait_object(_: &dyn BearerInterceptor) {}
    let _ = _assert_trait_object;
}
