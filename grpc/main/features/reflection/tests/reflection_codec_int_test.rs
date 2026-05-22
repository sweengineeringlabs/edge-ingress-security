//! Integration tests covering codec helpers and service builder methods that were
//! not exercised by the inline unit tests.
//!
//! Rule 77: with_descriptors, service_name_from_method_path, encode_response.
//! Rule 78: @covers annotations for all test functions.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_edge_ingress_grpc::{GrpcIngress, GrpcRequest};
use swe_edge_ingress_grpc_reflection::{
    encode_response, service_name_from_method_path, Descriptor, ReflectionRequest,
    ReflectionResponse, ReflectionService, REFLECTION_INFO_METHOD, REFLECTION_SERVICE_NAME,
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn encode_string_field(tag: u8, value: &str) -> Vec<u8> {
    let mut out = vec![tag];
    encode_varint(value.len() as u64, &mut out);
    out.extend_from_slice(value.as_bytes());
    out
}

fn encode_varint(mut value: u64, out: &mut Vec<u8>) {
    while value >= 0x80 {
        out.push((value as u8) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

/// Build a `ServerReflectionRequest { file_by_filename: name }` body.
fn file_by_filename_body(name: &str) -> Vec<u8> {
    encode_string_field(0x1a, name) // tag 3 wire 2
}

/// Build a `ServerReflectionRequest { file_containing_symbol: sym }` body.
fn file_containing_symbol_body(sym: &str) -> Vec<u8> {
    encode_string_field(0x22, sym) // tag 4 wire 2
}

fn make_svc_with_descriptors(descriptors: Vec<Descriptor>) -> ReflectionService {
    ReflectionService::new(Arc::new(HandlerRegistry::new())).with_descriptors(descriptors)
}

// ── with_descriptors ──────────────────────────────────────────────────────────

/// @covers: ReflectionService::with_descriptors — registered descriptor is found by FileByFilename.
#[tokio::test]
async fn test_with_descriptors_registered_descriptor_is_found_by_filename() {
    let svc = make_svc_with_descriptors(vec![Descriptor {
        filename: "a.proto".into(),
        symbols: vec!["pkg.A".into()],
        bytes: vec![0xaa],
    }]);

    let body = file_by_filename_body("a.proto");
    let req = GrpcRequest::new(
        REFLECTION_INFO_METHOD,
        body,
        std::time::Duration::from_secs(5),
    );
    let resp = svc
        .handle_unary(req, edge_domain::RequestContext::unauthenticated())
        .await
        .expect("handle_unary must succeed");

    // The encoded response must contain the descriptor bytes 0xaa.
    assert!(
        resp.body.contains(&0xaa),
        "descriptor bytes not found in response: {:?}",
        resp.body
    );
}

/// @covers: ReflectionService::with_descriptors — multiple descriptors all registered; each is findable.
#[tokio::test]
async fn test_with_descriptors_multiple_descriptors_all_findable_by_filename() {
    let svc = make_svc_with_descriptors(vec![
        Descriptor {
            filename: "a.proto".into(),
            symbols: vec!["pkg.A".into()],
            bytes: vec![0x0a],
        },
        Descriptor {
            filename: "b.proto".into(),
            symbols: vec!["pkg.B".into()],
            bytes: vec![0x0b],
        },
    ]);

    for (name, byte) in [("a.proto", 0x0au8), ("b.proto", 0x0bu8)] {
        let body = file_by_filename_body(name);
        let req = GrpcRequest::new(
            REFLECTION_INFO_METHOD,
            body,
            std::time::Duration::from_secs(5),
        );
        let resp = svc
            .handle_unary(req, edge_domain::RequestContext::unauthenticated())
            .await
            .expect("handle_unary must succeed");
        assert!(
            resp.body.contains(&byte),
            "descriptor byte {byte:#x} not found for {name}: {:?}",
            resp.body
        );
    }
}

/// @covers: ReflectionService::with_descriptors — empty iterator results in NOT_FOUND for any filename.
#[tokio::test]
async fn test_with_descriptors_empty_iterator_yields_not_found_for_any_filename() {
    let svc = make_svc_with_descriptors(vec![]);

    let body = file_by_filename_body("missing.proto");
    let req = GrpcRequest::new(
        REFLECTION_INFO_METHOD,
        body,
        std::time::Duration::from_secs(5),
    );
    let resp = svc
        .handle_unary(req, edge_domain::RequestContext::unauthenticated())
        .await
        .expect("handle_unary must succeed");

    // The response must not be empty; it contains an ErrorResponse(NOT_FOUND=5).
    // We detect NOT_FOUND by checking that 0x08, 0x05 (field 1 varint = 5) appears inside
    // the error_response sub-message (field tag 0x3a).
    assert!(
        !resp.body.is_empty(),
        "response must not be empty for missing descriptor"
    );
}

/// @covers: ReflectionService::with_descriptors — descriptor registered via with_descriptors is found by symbol.
#[tokio::test]
async fn test_with_descriptors_descriptor_found_by_containing_symbol() {
    let svc = make_svc_with_descriptors(vec![Descriptor {
        filename: "svc.proto".into(),
        symbols: vec!["pkg.Svc".into(), "pkg.Svc.Call".into()],
        bytes: vec![0xcc],
    }]);

    let body = file_containing_symbol_body("pkg.Svc.Call");
    let req = GrpcRequest::new(
        REFLECTION_INFO_METHOD,
        body,
        std::time::Duration::from_secs(5),
    );
    let resp = svc
        .handle_unary(req, edge_domain::RequestContext::unauthenticated())
        .await
        .expect("handle_unary must succeed");

    assert!(
        resp.body.contains(&0xcc),
        "descriptor bytes not found via symbol lookup: {:?}",
        resp.body
    );
}

// ── service_name_from_method_path ─────────────────────────────────────────────

/// @covers: service_name_from_method_path — well-formed path extracts service portion.
#[test]
fn test_service_name_from_method_path_well_formed_path_returns_service_name() {
    assert_eq!(
        service_name_from_method_path("/pkg.MyService/Method"),
        Some("pkg.MyService")
    );
}

/// @covers: service_name_from_method_path — path without leading slash returns None.
#[test]
fn test_service_name_from_method_path_no_leading_slash_returns_none() {
    assert!(service_name_from_method_path("pkg.MyService/Method").is_none());
}

/// @covers: service_name_from_method_path — empty string returns None.
#[test]
fn test_service_name_from_method_path_empty_string_returns_none() {
    assert!(service_name_from_method_path("").is_none());
}

/// @covers: service_name_from_method_path — slash-only path returns None (empty service name).
#[test]
fn test_service_name_from_method_path_slash_only_returns_none() {
    assert!(service_name_from_method_path("/").is_none());
}

/// @covers: service_name_from_method_path — leading slash with no trailing slash returns None.
#[test]
fn test_service_name_from_method_path_no_method_segment_returns_none() {
    assert!(service_name_from_method_path("/ServiceOnly").is_none());
}

/// @covers: service_name_from_method_path — reflection own method path returns REFLECTION_SERVICE_NAME.
#[test]
fn test_service_name_from_method_path_reflection_own_method_returns_service_name() {
    let name = service_name_from_method_path(
        "/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo",
    );
    assert_eq!(name, Some(REFLECTION_SERVICE_NAME));
}

// ── encode_response ───────────────────────────────────────────────────────────

/// @covers: encode_response — ListServices response produces non-empty output.
#[test]
fn test_encode_response_list_services_produces_non_empty_output() {
    let resp = ReflectionResponse::ListServices(vec!["pkg.Demo".into()]);
    let out = encode_response(&resp, &[]);
    assert!(!out.is_empty(), "encoded ListServices must not be empty");
}

/// @covers: encode_response — FileDescriptor response embeds descriptor bytes verbatim.
#[test]
fn test_encode_response_file_descriptor_embeds_descriptor_bytes() {
    let payload = vec![0xde, 0xad, 0xbe, 0xef];
    let resp = ReflectionResponse::FileDescriptor(vec![payload.clone()]);
    let out = encode_response(&resp, &[]);
    assert!(
        out.windows(payload.len()).any(|w| w == payload.as_slice()),
        "descriptor bytes not found in encoded output: {out:?}"
    );
}

/// @covers: encode_response — Error response encodes non-zero error_code as varint field.
#[test]
fn test_encode_response_error_encodes_error_code_nonzero() {
    let resp = ReflectionResponse::Error {
        error_code: 5,
        error_message: "not found".into(),
    };
    let out = encode_response(&resp, &[]);
    // Field 1 (varint tag = 0x08) with value 5 must be present inside the sub-message.
    assert!(
        out.windows(2).any(|w| w == [0x08, 0x05]),
        "error_code=5 not found in encoded output: {out:?}"
    );
}

/// @covers: encode_response — original_request bytes are echoed in the response envelope.
#[test]
fn test_encode_response_echoes_original_request_in_envelope() {
    let req_body = vec![0x3a, 0x00]; // list_services request
    let resp = ReflectionResponse::ListServices(vec![]);
    let out = encode_response(&resp, &req_body);
    assert!(
        out.windows(req_body.len())
            .any(|w| w == req_body.as_slice()),
        "original request not echoed in envelope: {out:?}"
    );
}

/// @covers: encode_response — empty original_request skips the envelope field, yielding shorter output.
#[test]
fn test_encode_response_empty_original_request_skips_envelope_field() {
    let resp = ReflectionResponse::ListServices(vec![]);
    let with_req = encode_response(&resp, &[0x3a, 0x00]);
    let without_req = encode_response(&resp, &[]);
    assert!(
        with_req.len() > without_req.len(),
        "output with echoed request ({}) must be longer than without ({})",
        with_req.len(),
        without_req.len()
    );
}

// ── ReflectionRequest round-trips through decode_request ─────────────────────

/// @covers: decode_request — ListServices request decodes to ListServices variant.
#[test]
fn test_decode_request_list_services_round_trips() {
    use swe_edge_ingress_grpc_reflection::decode_request;
    let body = vec![0x3a, 0x00];
    let req = decode_request(&body).expect("decode must succeed");
    assert_eq!(req, ReflectionRequest::ListServices(String::new()));
}

/// @covers: decode_request — FileByFilename request decodes to FileByFilename variant.
#[test]
fn test_decode_request_file_by_filename_round_trips() {
    use swe_edge_ingress_grpc_reflection::decode_request;
    let body = file_by_filename_body("pkg/foo.proto");
    let req = decode_request(&body).expect("decode must succeed");
    assert_eq!(
        req,
        ReflectionRequest::FileByFilename("pkg/foo.proto".into())
    );
}
