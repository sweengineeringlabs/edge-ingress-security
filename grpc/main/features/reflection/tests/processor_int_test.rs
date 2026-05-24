//! Integration tests for the `Processor` contract surface.
//!
//! Rule 105: covers the `Processor` trait behaviour via the `handle_reflection`
//! SAF wrapper, which is the public equivalent of calling `Processor::process`.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_edge_ingress_grpc_reflection::{
    handle_reflection, ReflectionRequest, ReflectionResponse, ReflectionService,
    REFLECTION_SERVICE_NAME,
};

fn empty_svc() -> ReflectionService {
    ReflectionService::new(Arc::new(HandlerRegistry::new()))
}

/// @covers: handle_reflection / Processor::process — ListServices request returns ListServices response.
#[test]
fn test_handle_reflection_list_services_returns_list_services_response() {
    let svc = empty_svc();
    let resp = handle_reflection(&svc, ReflectionRequest::ListServices(String::new()));
    assert!(
        matches!(resp, ReflectionResponse::ListServices(_)),
        "expected ListServices response, got {resp:?}"
    );
}

/// @covers: handle_reflection / Processor::process — ListServices always includes reflection self-name.
#[test]
fn test_handle_reflection_list_services_includes_reflection_self_name() {
    let svc = empty_svc();
    let resp = handle_reflection(&svc, ReflectionRequest::ListServices(String::new()));
    match resp {
        ReflectionResponse::ListServices(names) => {
            assert!(
                names.iter().any(|n| n == REFLECTION_SERVICE_NAME),
                "reflection self-name missing from ListServices: {names:?}"
            );
        }
        other => panic!("expected ListServices, got {other:?}"),
    }
}

/// @covers: handle_reflection / Processor::process — Unknown request returns Error with INVALID_ARGUMENT code.
#[test]
fn test_handle_reflection_unknown_request_returns_invalid_argument_error() {
    use swe_edge_ingress_grpc_reflection::ERROR_CODE_INVALID_ARGUMENT;
    let svc = empty_svc();
    let resp = handle_reflection(&svc, ReflectionRequest::Unknown);
    match resp {
        ReflectionResponse::Error { error_code, .. } => {
            assert_eq!(
                error_code, ERROR_CODE_INVALID_ARGUMENT,
                "expected INVALID_ARGUMENT({ERROR_CODE_INVALID_ARGUMENT}), got {error_code}"
            );
        }
        other => panic!("expected Error variant, got {other:?}"),
    }
}

/// @covers: handle_reflection / Processor::process — FileContainingExtension returns UNIMPLEMENTED.
#[test]
fn test_handle_reflection_file_containing_extension_returns_unimplemented_error() {
    use swe_edge_ingress_grpc_reflection::ERROR_CODE_UNIMPLEMENTED;
    let svc = empty_svc();
    let resp = handle_reflection(
        &svc,
        ReflectionRequest::FileContainingExtension {
            containing_type: "pkg.Msg".into(),
            extension_number: 42,
        },
    );
    match resp {
        ReflectionResponse::Error { error_code, .. } => {
            assert_eq!(
                error_code, ERROR_CODE_UNIMPLEMENTED,
                "expected UNIMPLEMENTED({ERROR_CODE_UNIMPLEMENTED}), got {error_code}"
            );
        }
        other => panic!("expected Error variant, got {other:?}"),
    }
}

/// @covers: handle_reflection / Processor::process — AllExtensionNumbersOfType returns UNIMPLEMENTED.
#[test]
fn test_handle_reflection_all_extension_numbers_returns_unimplemented_error() {
    use swe_edge_ingress_grpc_reflection::ERROR_CODE_UNIMPLEMENTED;
    let svc = empty_svc();
    let resp = handle_reflection(
        &svc,
        ReflectionRequest::AllExtensionNumbersOfType("pkg.Msg".into()),
    );
    match resp {
        ReflectionResponse::Error { error_code, .. } => {
            assert_eq!(
                error_code, ERROR_CODE_UNIMPLEMENTED,
                "expected UNIMPLEMENTED, got {error_code}"
            );
        }
        other => panic!("expected Error variant, got {other:?}"),
    }
}
