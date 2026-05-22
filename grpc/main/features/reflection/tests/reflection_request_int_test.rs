//! Integration tests for `ReflectionRequest` variants and their behaviour.

use swe_edge_ingress_grpc_reflection::ReflectionRequest;

/// @covers: ReflectionRequest — PartialEq correctly distinguishes same-variant same-value.
#[test]
fn test_reflection_request_partial_eq_same_variant_same_value_are_equal() {
    let a = ReflectionRequest::FileByFilename("a.proto".into());
    let b = ReflectionRequest::FileByFilename("a.proto".into());
    assert_eq!(a, b);
}

/// @covers: ReflectionRequest — PartialEq distinguishes same variant different values.
#[test]
fn test_reflection_request_partial_eq_same_variant_different_value_are_not_equal() {
    let a = ReflectionRequest::FileByFilename("a.proto".into());
    let b = ReflectionRequest::FileByFilename("b.proto".into());
    assert_ne!(a, b);
}

/// @covers: ReflectionRequest — PartialEq distinguishes different variants.
#[test]
fn test_reflection_request_partial_eq_different_variants_are_not_equal() {
    let a = ReflectionRequest::FileByFilename("x.proto".into());
    let b = ReflectionRequest::FileContainingSymbol("x.proto".into());
    assert_ne!(a, b);
}

/// @covers: ReflectionRequest::Unknown — Unknown variant equals Unknown variant.
#[test]
fn test_reflection_request_unknown_equals_unknown() {
    assert_eq!(ReflectionRequest::Unknown, ReflectionRequest::Unknown);
}

/// @covers: ReflectionRequest::FileContainingExtension — struct variant preserves both fields.
#[test]
fn test_reflection_request_file_containing_extension_preserves_both_fields() {
    let req = ReflectionRequest::FileContainingExtension {
        containing_type: "pkg.Msg".into(),
        extension_number: 42,
    };
    match req {
        ReflectionRequest::FileContainingExtension {
            containing_type,
            extension_number,
        } => {
            assert_eq!(containing_type, "pkg.Msg");
            assert_eq!(extension_number, 42);
        }
        other => panic!("expected FileContainingExtension, got {other:?}"),
    }
}

/// @covers: ReflectionRequest::ListServices — Clone produces equal value.
#[test]
fn test_reflection_request_list_services_clone_produces_equal_value() {
    let original = ReflectionRequest::ListServices("".into());
    let cloned = original.clone();
    assert_eq!(original, cloned);
}
