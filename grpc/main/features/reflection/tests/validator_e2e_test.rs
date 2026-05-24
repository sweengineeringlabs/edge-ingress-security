//! Integration tests for the validation pathway.
//!
//! Rule 105: test file for the `Validator` contract via the `validate_payload` SAF wrapper.

use swe_edge_ingress_grpc_reflection::validate_payload;

/// @covers: validate_payload — accepts empty payload.
#[test]
fn test_validate_payload_accepts_empty_payload() {
    let result = validate_payload(&[]);
    assert!(
        result.is_ok(),
        "default validator should accept empty payload"
    );
}

/// @covers: validate_payload — accepts non-empty payload.
#[test]
fn test_validate_payload_accepts_non_empty_bytes() {
    let result = validate_payload(&[0x01, 0x02, 0x03]);
    assert!(
        result.is_ok(),
        "default validator should accept non-empty payload"
    );
}

/// @covers: validate_payload — accepts large payload without panic.
#[test]
fn test_validate_payload_accepts_large_payload() {
    let payload = vec![0xffu8; 65536];
    let result = validate_payload(&payload);
    assert!(
        result.is_ok(),
        "default validator should accept large payload"
    );
}
