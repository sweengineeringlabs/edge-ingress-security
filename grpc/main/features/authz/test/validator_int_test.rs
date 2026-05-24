//! Integration tests for the [`Validator`] trait contract.

use swe_edge_ingress_grpc_authz::{validate_application_config, ApplicationConfig};

/// @covers: validate_application_config
#[test]
fn authz_struct_application_config_validator_allows_allow_policy_int_test() {
    let cfg = ApplicationConfig {
        default_policy: "allow".into(),
    };
    assert!(
        validate_application_config(&cfg).is_ok(),
        "allow policy must pass validation"
    );
}

/// @covers: validate_application_config
#[test]
fn authz_struct_application_config_validator_rejects_empty_policy_int_test() {
    let cfg = ApplicationConfig {
        default_policy: String::new(),
    };
    assert!(
        validate_application_config(&cfg).is_err(),
        "empty policy must fail validation"
    );
}
