//! Integration tests for `src/saf/reflection_svc.rs`.
//!
//! Rule 120: test coverage for the SAF reflection service wrapper.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_edge_ingress_grpc_reflection::{
    handle_reflection, validate_payload, ReflectionRequest, ReflectionResponse, ReflectionService,
};

fn empty_svc() -> ReflectionService {
    ReflectionService::new(Arc::new(HandlerRegistry::new()))
}

/// @covers: handle_reflection
#[test]
fn test_handle_reflection_list_services_returns_list_services_variant() {
    let svc = empty_svc();
    let resp = handle_reflection(&svc, ReflectionRequest::ListServices(String::new()));
    assert!(
        matches!(resp, ReflectionResponse::ListServices(_)),
        "expected ListServices response, got {resp:?}"
    );
}

/// @covers: validate_payload
#[test]
fn test_validate_payload_accepts_any_byte_slice() {
    assert!(validate_payload(&[]).is_ok());
    assert!(validate_payload(&[0xff, 0xfe, 0xfd]).is_ok());
}
