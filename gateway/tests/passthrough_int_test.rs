//! Integration tests for the passthrough validator via the SAF.

use swe_edge_ingress::{passthrough_validator, Validator};

/// @covers: passthrough_validator — accepts any non-empty string.
#[test]
fn test_passthrough_validator_accepts_non_empty_input() {
    let v = passthrough_validator();
    assert!(
        v.is_valid("hello world"),
        "passthrough validator must accept any non-empty input",
    );
}

/// @covers: passthrough_validator — accepts empty string unconditionally.
#[test]
fn test_passthrough_validator_accepts_empty_string() {
    let v = passthrough_validator();
    assert!(
        v.is_valid(""),
        "passthrough validator must accept empty string",
    );
}

/// @covers: passthrough_validator — accepts special characters.
#[test]
fn test_passthrough_validator_accepts_special_characters() {
    let v = passthrough_validator();
    assert!(
        v.is_valid("!@#$%^&*()_+-=[]{}|;':\",./<>?"),
        "passthrough validator must accept special characters",
    );
}
