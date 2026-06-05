//! Integration tests — Validator trait contract.

use swe_edge_ingress_message_broker_transport::Validator;

struct AlwaysOk;
impl Validator for AlwaysOk {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

struct AlwaysFail;
impl Validator for AlwaysFail {
    fn validate(&self) -> Result<(), String> {
        Err("forced error".into())
    }
}

/// @covers: Validator::validate — Ok branch
#[test]
fn test_validator_ok_returns_unit() {
    assert!(AlwaysOk.validate().is_ok());
}

/// @covers: Validator::validate — Err branch
#[test]
fn test_validator_err_returns_error_string() {
    assert!(AlwaysFail.validate().is_err());
}
