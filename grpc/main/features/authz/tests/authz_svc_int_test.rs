//! Integration tests for SAF-level authz service introspection.

use swe_edge_ingress_grpc::PeerIdentity;
use swe_edge_ingress_grpc_authz::{
    assert_is_processor, is_authorization_interceptor, validate_application_config,
    ApplicationConfig, AuthzInterceptor,
};

/// @covers: is_authorization_interceptor
#[test]
fn authz_struct_svc_is_authorization_interceptor_returns_true_int_test() {
    assert!(
        is_authorization_interceptor(),
        "is_authorization_interceptor must return true — \
         the authz crate's interceptor is always an authorization gate"
    );
}

/// @covers: validate_application_config
#[test]
fn authz_struct_svc_validate_application_config_accepts_deny_policy_int_test() {
    let cfg = ApplicationConfig {
        default_policy: "deny".into(),
    };
    assert!(validate_application_config(&cfg).is_ok());
}

/// @covers: validate_application_config
#[test]
fn authz_struct_svc_validate_application_config_rejects_unknown_policy_int_test() {
    let cfg = ApplicationConfig {
        default_policy: "unknown".into(),
    };
    assert!(validate_application_config(&cfg).is_err());
}

/// @covers: assert_is_processor
#[test]
fn authz_struct_interceptor_assert_is_processor_accepts_authz_interceptor_int_test() {
    let interceptor = AuthzInterceptor::from_policy(|_: &PeerIdentity, _: &str| true);
    assert_is_processor(&interceptor);
}
