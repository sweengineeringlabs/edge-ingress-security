//! Integration tests for [`ApplicationConfig`].

use swe_edge_ingress_grpc_authz::{validate_application_config, ApplicationConfig};

/// @covers: validate_application_config
#[test]
fn authz_struct_application_config_default_policy_is_deny_int_test() {
    let cfg = ApplicationConfig::default();
    assert_eq!(cfg.default_policy, "deny");
    assert!(validate_application_config(&cfg).is_ok());
}

/// @covers: validate_application_config
#[test]
fn authz_struct_application_config_allow_policy_passes_validation_int_test() {
    let cfg = ApplicationConfig {
        default_policy: "allow".into(),
    };
    assert!(validate_application_config(&cfg).is_ok());
}
