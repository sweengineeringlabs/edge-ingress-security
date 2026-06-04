//! Public API tests for reflection SAF layer.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_edge_ingress_grpc_reflection::{
    create_config_builder, handle_reflection, validate_payload, ReflectionRequest,
    ReflectionResponse, ReflectionService,
};

fn make_registry() -> Arc<HandlerRegistry<Vec<u8>, Vec<u8>>> {
    Arc::new(HandlerRegistry::new())
}

#[test]
fn test_create_config_builder_returns_builder_with_name_and_version() {
    let builder = create_config_builder();
    assert_eq!(builder.name(), "swe-edge-configbuilder");
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_handle_reflection_list_services_returns_response() {
    let svc = ReflectionService::new(make_registry());
    let req = ReflectionRequest::ListServices(String::new());
    let resp = handle_reflection(&svc, req);
    assert!(
        matches!(resp, ReflectionResponse::ListServices(_)),
        "ListServices request must produce ListServices response"
    );
}

#[test]
fn test_validate_payload_accepts_empty_bytes() {
    let result = validate_payload(&[]);
    assert!(
        result.is_ok(),
        "default validator must accept empty payload"
    );
}

#[test]
fn test_validate_payload_accepts_arbitrary_bytes() {
    let result = validate_payload(&[0xde, 0xad, 0xbe, 0xef]);
    assert!(
        result.is_ok(),
        "default validator must accept arbitrary bytes"
    );
}
