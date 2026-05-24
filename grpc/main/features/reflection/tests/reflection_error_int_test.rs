//! Integration tests for `ReflectionError` variants and their `Display` output.

use swe_edge_ingress_grpc_reflection::ReflectionError;

/// @covers: ReflectionError::Malformed — Display includes the detail string.
#[test]
fn test_reflection_error_malformed_display_includes_detail_string() {
    let err = ReflectionError::Malformed("varint overflow".into());
    let msg = err.to_string();
    assert!(
        msg.contains("varint overflow"),
        "expected 'varint overflow' in display, got: {msg}"
    );
}

/// @covers: ReflectionError::UnknownRequest — Display includes the field number.
#[test]
fn test_reflection_error_unknown_request_display_includes_field_number() {
    let err = ReflectionError::UnknownRequest(42);
    let msg = err.to_string();
    assert!(msg.contains("42"), "expected '42' in display, got: {msg}");
}

/// @covers: ReflectionError::Malformed — Debug output is non-empty.
#[test]
fn test_reflection_error_malformed_debug_output_is_non_empty() {
    let err = ReflectionError::Malformed("bad bytes".into());
    assert!(!format!("{err:?}").is_empty());
}

/// @covers: ReflectionError::UnknownRequest — field number 0 is valid (boundary).
#[test]
fn test_reflection_error_unknown_request_field_zero_is_valid() {
    let err = ReflectionError::UnknownRequest(0);
    let msg = err.to_string();
    assert!(msg.contains('0'), "expected '0' in display, got: {msg}");
}

/// @covers: ReflectionError::Malformed — empty detail string is accepted without panic.
#[test]
fn test_reflection_error_malformed_empty_detail_does_not_panic() {
    let err = ReflectionError::Malformed(String::new());
    let _msg = err.to_string();
}
