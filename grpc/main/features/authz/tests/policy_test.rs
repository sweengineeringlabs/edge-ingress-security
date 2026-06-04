//! Tests for method/acl/policy API mirror module.

/// @covers: MethodAclPolicy
#[test]
fn test_method_acl_policy_struct_is_publicly_exported() {
    let _ = std::any::type_name::<swe_edge_ingress_grpc_authz::MethodAclPolicy>();
}
