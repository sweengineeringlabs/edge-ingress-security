//! Integration tests for `ReflectionResponse` variants and their behaviour.

use swe_edge_ingress_grpc_reflection::ReflectionResponse;

/// @covers: ReflectionResponse::ListServices — variant carries the service name list.
#[test]
fn test_reflection_response_list_services_carries_service_name_list() {
    let resp = ReflectionResponse::ListServices(vec!["pkg.Demo".into(), "pkg.Other".into()]);
    match resp {
        ReflectionResponse::ListServices(names) => {
            assert_eq!(names.len(), 2);
            assert_eq!(names[0], "pkg.Demo");
            assert_eq!(names[1], "pkg.Other");
        }
        other => panic!("expected ListServices, got {other:?}"),
    }
}

/// @covers: ReflectionResponse::FileDescriptor — variant carries raw bytes verbatim.
#[test]
fn test_reflection_response_file_descriptor_carries_bytes_verbatim() {
    let bytes = vec![vec![0xde, 0xad], vec![0xbe, 0xef]];
    let resp = ReflectionResponse::FileDescriptor(bytes.clone());
    match resp {
        ReflectionResponse::FileDescriptor(files) => {
            assert_eq!(files, bytes);
        }
        other => panic!("expected FileDescriptor, got {other:?}"),
    }
}

/// @covers: ReflectionResponse::Error — variant carries error_code and error_message.
#[test]
fn test_reflection_response_error_carries_code_and_message() {
    let resp = ReflectionResponse::Error {
        error_code: 5,
        error_message: "not found".into(),
    };
    match resp {
        ReflectionResponse::Error {
            error_code,
            error_message,
        } => {
            assert_eq!(error_code, 5);
            assert_eq!(error_message, "not found");
        }
        other => panic!("expected Error, got {other:?}"),
    }
}

/// @covers: ReflectionResponse — PartialEq distinguishes different variants.
#[test]
fn test_reflection_response_partial_eq_distinguishes_different_variants() {
    let a = ReflectionResponse::ListServices(vec![]);
    let b = ReflectionResponse::FileDescriptor(vec![]);
    assert_ne!(a, b);
}

/// @covers: ReflectionResponse — Clone produces an equal value.
#[test]
fn test_reflection_response_clone_produces_equal_value() {
    let original = ReflectionResponse::ListServices(vec!["a".into()]);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}
