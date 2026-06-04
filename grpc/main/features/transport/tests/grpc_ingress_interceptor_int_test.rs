//! Tests for GrpcIngressInterceptor API module.

use swe_edge_ingress_grpc_transport::GrpcIngressInterceptor;

#[test]
fn test_grpc_ingress_interceptor_trait_is_available() {
    // Verify GrpcIngressInterceptor trait is accessible.
    fn _assert_trait_object(_: &dyn GrpcIngressInterceptor) {}
    let _ = _assert_trait_object;
}
