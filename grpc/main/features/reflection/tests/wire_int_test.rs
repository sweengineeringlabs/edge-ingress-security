//! Integration tests for codec helpers `ReflectionCodec::decode_request` and `encode_response`.
//!
//! Rules 77 + 78: covers the wire codec with `/// @covers:` annotations.

use swe_edge_ingress_grpc_reflection::{ReflectionCodec, ReflectionRequest, ReflectionResponse};

// ── helpers ───────────────────────────────────────────────────────────────────

fn encode_varint(mut value: u64, out: &mut Vec<u8>) {
    while value >= 0x80 {
        out.push((value as u8) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn string_field(tag: u8, value: &str) -> Vec<u8> {
    let mut out = vec![tag];
    encode_varint(value.len() as u64, &mut out);
    out.extend_from_slice(value.as_bytes());
    out
}

// ── ReflectionCodec::decode_request ────────────────────────────────────────────────────────────

/// @covers: ReflectionCodec::decode_request — empty body decodes to Unknown variant.
#[test]
fn test_decode_request_empty_body_returns_unknown() {
    let result = ReflectionCodec::decode_request(&[]).expect("decode must not error on empty body");
    assert_eq!(result, ReflectionRequest::Unknown);
}

/// @covers: ReflectionCodec::decode_request — list_services field (tag 7, wire 2) decodes correctly.
#[test]
fn test_decode_request_list_services_field_decodes_to_list_services_variant() {
    let body = vec![0x3a, 0x00]; // field 7 wire 2, length 0
    let result = ReflectionCodec::decode_request(&body).expect("decode must succeed");
    assert_eq!(result, ReflectionRequest::ListServices(String::new()));
}

/// @covers: ReflectionCodec::decode_request — file_by_filename field (tag 3, wire 2) decodes to correct filename.
#[test]
fn test_decode_request_file_by_filename_decodes_filename_correctly() {
    let body = string_field(0x1a, "google/protobuf/empty.proto");
    let result = ReflectionCodec::decode_request(&body).expect("decode must succeed");
    assert_eq!(
        result,
        ReflectionRequest::FileByFilename("google/protobuf/empty.proto".into())
    );
}

/// @covers: ReflectionCodec::decode_request — file_containing_symbol field (tag 4, wire 2) decodes symbol.
#[test]
fn test_decode_request_file_containing_symbol_decodes_symbol_correctly() {
    let body = string_field(0x22, "pkg.MyService");
    let result = ReflectionCodec::decode_request(&body).expect("decode must succeed");
    assert_eq!(
        result,
        ReflectionRequest::FileContainingSymbol("pkg.MyService".into())
    );
}

/// @covers: ReflectionCodec::decode_request — all_extension_numbers_of_type field (tag 6, wire 2) decodes type name.
#[test]
fn test_decode_request_all_extension_numbers_decodes_type_name() {
    let body = string_field(0x32, "pkg.Extendable");
    let result = ReflectionCodec::decode_request(&body).expect("decode must succeed");
    assert_eq!(
        result,
        ReflectionRequest::AllExtensionNumbersOfType("pkg.Extendable".into())
    );
}

/// @covers: ReflectionCodec::decode_request — malformed varint (truncated body) returns Malformed error.
#[test]
fn test_decode_request_truncated_varint_returns_malformed_error() {
    // 0x80 alone is an incomplete varint (continuation bit set, no next byte)
    let body = vec![0x1a, 0x80];
    let result = ReflectionCodec::decode_request(&body);
    assert!(result.is_err(), "expected Err for truncated varint, got Ok");
}

// ── encode_response ───────────────────────────────────────────────────────────

/// @covers: encode_response — ListServices with one name produces non-empty bytes.
#[test]
fn test_encode_response_list_services_one_name_produces_non_empty_bytes() {
    let resp = ReflectionResponse::ListServices(vec!["grpc.health.v1.Health".into()]);
    let out = ReflectionCodec::encode_response(&resp, &[]);
    assert!(!out.is_empty(), "encoded ListServices must not be empty");
}

/// @covers: encode_response — FileDescriptor bytes appear verbatim in encoded output.
#[test]
fn test_encode_response_file_descriptor_bytes_appear_verbatim_in_output() {
    let payload = vec![0xca, 0xfe, 0xba, 0xbe];
    let resp = ReflectionResponse::FileDescriptor(vec![payload.clone()]);
    let out = ReflectionCodec::encode_response(&resp, &[]);
    assert!(
        out.windows(payload.len()).any(|w| w == payload.as_slice()),
        "descriptor bytes not found verbatim in output: {out:?}"
    );
}

/// @covers: encode_response — Error variant encodes error_code as varint field 1.
#[test]
fn test_encode_response_error_variant_encodes_error_code_field() {
    let resp = ReflectionResponse::Error {
        error_code: 12,
        error_message: "unimplemented".into(),
    };
    let out = ReflectionCodec::encode_response(&resp, &[]);
    // Field 1 varint tag = 0x08, value 12 = 0x0c
    assert!(
        out.windows(2).any(|w| w == [0x08, 0x0c]),
        "error_code=12 not found in encoded output: {out:?}"
    );
}

/// @covers: encode_response — original_request bytes are echoed in the response envelope.
#[test]
fn test_encode_response_original_request_echoed_in_response_envelope() {
    let req_body = vec![0x3a, 0x00];
    let resp = ReflectionResponse::ListServices(vec![]);
    let out = ReflectionCodec::encode_response(&resp, &req_body);
    assert!(
        out.windows(req_body.len())
            .any(|w| w == req_body.as_slice()),
        "original request bytes not found in envelope: {out:?}"
    );
}

/// @covers: encode_response — empty original_request skips envelope field, shorter output.
#[test]
fn test_encode_response_empty_original_request_produces_shorter_output() {
    let resp = ReflectionResponse::ListServices(vec![]);
    let with_echo = ReflectionCodec::encode_response(&resp, &[0x3a, 0x00]);
    let without_echo = ReflectionCodec::encode_response(&resp, &[]);
    assert!(
        with_echo.len() > without_echo.len(),
        "output with echo ({}) must be longer than without ({})",
        with_echo.len(),
        without_echo.len()
    );
}

/// @covers: encode_response — ListServices with zero names produces output that is still valid (non-panic).
#[test]
fn test_encode_response_list_services_empty_names_does_not_panic() {
    let resp = ReflectionResponse::ListServices(vec![]);
    let _out = ReflectionCodec::encode_response(&resp, &[]);
}
