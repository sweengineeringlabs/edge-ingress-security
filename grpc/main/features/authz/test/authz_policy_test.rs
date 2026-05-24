//! Tests for authz_policy API module.

use swe_edge_ingress_grpc_authz::AuthzPolicy;

#[test]
fn test_authz_policy_trait_is_available() {
    // Verify AuthzPolicy trait is accessible from api layer.
    fn _assert_trait_object(_: &dyn AuthzPolicy) {}
    let _ = _assert_trait_object;
}
