//! Integration tests for the `Validator` trait contract.
//!
//! Rule 120: test coverage for `src/api/traits.rs`.

use swe_edge_ingress_grpc_reflection::validate_payload;

/// @covers: validate_payload
#[test]
fn test_traits_validator_contract_accepts_well_formed_payload() {
    assert!(
        validate_payload(&[0x0a, 0x03, 0x61, 0x62, 0x63]).is_ok(),
        "Validator contract must accept well-formed payload"
    );
}

/// @covers: validate_payload
#[test]
fn test_traits_validator_contract_accepts_empty_payload() {
    assert!(
        validate_payload(&[]).is_ok(),
        "Validator contract must accept empty payload"
    );
}
