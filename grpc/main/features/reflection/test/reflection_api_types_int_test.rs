//! Integration tests for `swe-edge-ingress-grpc-reflection` public API types and error variants.
//!
//! Covers: `ReflectionError`, `ReflectionRequest`, `ReflectionResponse`, `Descriptor`.

use swe_edge_ingress_grpc_reflection::{
    Descriptor, ReflectionError, ReflectionRequest, ReflectionResponse,
};

// ── ReflectionError ───────────────────────────────────────────────────────────

/// @covers: ReflectionError::Malformed — display includes the detail message.
#[test]
fn test_malformed_displays_message() {
    let e = ReflectionError::Malformed("truncated body".into());
    assert!(
        e.to_string().contains("truncated body"),
        "expected 'truncated body' in display, got: {}",
        e
    );
}

/// @covers: ReflectionError::UnknownRequest — display includes the field number.
#[test]
fn test_unknown_request_displays_field_number() {
    let e = ReflectionError::UnknownRequest(99);
    assert!(
        e.to_string().contains("99"),
        "expected '99' in display, got: {}",
        e
    );
}

// ── ReflectionRequest ─────────────────────────────────────────────────────────

/// @covers: ReflectionRequest::FileByFilename — PartialEq distinguishes different filenames.
#[test]
fn test_reflection_request_partial_eq_compares_file_by_filename() {
    let a = ReflectionRequest::FileByFilename("a.proto".into());
    let b = ReflectionRequest::FileByFilename("a.proto".into());
    let c = ReflectionRequest::FileByFilename("b.proto".into());
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// ── ReflectionResponse ────────────────────────────────────────────────────────

/// @covers: ReflectionResponse::Error — variant carries error_code and error_message.
#[test]
fn test_reflection_response_error_variant_carries_code_and_message() {
    let err = ReflectionResponse::Error {
        error_code: 5,
        error_message: "not found".into(),
    };
    match err {
        ReflectionResponse::Error {
            error_code,
            error_message,
        } => {
            assert_eq!(error_code, 5);
            assert_eq!(error_message, "not found");
        }
        _ => panic!("expected Error variant"),
    }
}

// ── Descriptor ────────────────────────────────────────────────────────────────

/// @covers: Descriptor — basic fields round-trip correctly after construction.
#[test]
fn test_descriptor_round_trips_basic_fields() {
    let d = Descriptor {
        filename: "pkg/foo.proto".into(),
        symbols: vec!["pkg.Foo".into(), "pkg.Foo.Bar".into()],
        bytes: vec![1, 2, 3],
    };
    assert_eq!(d.filename, "pkg/foo.proto");
    assert_eq!(d.symbols.len(), 2);
    assert_eq!(d.bytes, vec![1, 2, 3]);
}
