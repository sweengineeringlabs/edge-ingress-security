//! Integration tests for AuthorizationInterceptor.

use swe_edge_ingress_grpc_transport::{AuthorizationInterceptor, GrpcIngressError};
use swe_edge_ingress_grpc_transport::{GrpcRequest, GrpcResponse};

/// @covers: AuthorizationInterceptor
#[test]
fn test_authorization_interceptor_is_a_supertrait_of_grpc_ingress_interceptor() {
    use swe_edge_ingress_grpc_transport::GrpcIngressInterceptor;

    struct AlwaysAllow;
    impl GrpcIngressInterceptor for AlwaysAllow {
        fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
            Ok(())
        }
        fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
            Ok(())
        }
        fn is_authorization(&self) -> bool {
            true
        }
    }
    impl AuthorizationInterceptor for AlwaysAllow {}
    let _ = AlwaysAllow;
    // Verify is_authorization returns true as required by the contract
    let guard = AlwaysAllow;
    assert!(guard.is_authorization());
}
