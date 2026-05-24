//! Integration tests for [`MethodAclConfig`].

use swe_edge_ingress_grpc::PeerIdentity;
use swe_edge_ingress_grpc_authz::{AuthzPolicy, MethodAclConfig, MethodAclPolicy};

fn policy_from(cfg: MethodAclConfig) -> MethodAclPolicy {
    MethodAclPolicy::from_config(cfg)
}

fn identity(cn: &str) -> PeerIdentity {
    PeerIdentity {
        cn: Some(cn.into()),
        ..Default::default()
    }
}

/// @covers: deny_all
#[test]
fn authz_struct_method_acl_config_deny_all_rejects_any_subject_int_test() {
    let policy = policy_from(MethodAclConfig::deny_all());
    assert!(!policy.allows(&identity("alice"), "/svc/M"));
    assert!(!policy.allows(&PeerIdentity::default(), "/svc/M"));
}

/// @covers: allow
#[test]
fn authz_struct_method_acl_config_allow_grants_listed_method_to_subject_int_test() {
    let policy = policy_from(MethodAclConfig::deny_all().allow("alice", ["/svc/Read".to_string()]));
    assert!(policy.allows(&identity("alice"), "/svc/Read"));
    assert!(!policy.allows(&identity("alice"), "/svc/Write"));
    assert!(!policy.allows(&identity("bob"), "/svc/Read"));
}

/// @covers: allow_for_any_authenticated
#[test]
fn authz_struct_method_acl_config_allow_for_any_authenticated_grants_method_to_any_subject_int_test(
) {
    let policy = policy_from(MethodAclConfig::deny_all().allow_for_any_authenticated("/health"));
    assert!(policy.allows(&identity("alice"), "/health"));
    assert!(policy.allows(&identity("bob"), "/health"));
    // Unauthenticated (no CN) must still be denied.
    assert!(!policy.allows(&PeerIdentity::default(), "/health"));
}

/// @covers: from_toml
#[test]
fn authz_struct_method_acl_config_from_toml_parses_by_subject_table_int_test() {
    let src = r#"
        [by_subject]
        alice = ["/svc/Read"]
        "*"   = ["/health"]
    "#;
    let cfg = MethodAclConfig::from_toml(src).expect("valid toml");
    let policy = policy_from(cfg);
    assert!(policy.allows(&identity("alice"), "/svc/Read"));
    assert!(policy.allows(&identity("bob"), "/health"));
    assert!(!policy.allows(&PeerIdentity::default(), "/health"));
}
