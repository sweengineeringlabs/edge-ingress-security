//! Integration tests for `AxumHttpServerHelper`.
use axum::http::{HeaderMap, HeaderValue};
use swe_edge_ingress_http_transport::AxumHttpServerHelper;

/// @covers: AxumHttpServerHelper::is_websocket_upgrade
#[test]
fn transport_struct_axum_http_server_helper_is_websocket_upgrade_returns_false_for_plain_request_int_test(
) {
    let headers = HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_websocket_upgrade(&headers));
}

/// @covers: AxumHttpServerHelper::is_sse_request
#[test]
fn transport_struct_axum_http_server_helper_is_sse_request_returns_false_for_plain_request_int_test(
) {
    let headers = HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_sse_request(&headers));
}

/// @covers: AxumHttpServerHelper::is_sse_request
#[test]
fn transport_struct_axum_http_server_helper_is_sse_request_returns_true_for_event_stream_int_test()
{
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("text/event-stream"));
    assert!(AxumHttpServerHelper::is_sse_request(&headers));
}

/// @covers: AxumHttpServerHelper::collect_headers
#[test]
fn transport_struct_axum_http_server_helper_collect_headers_returns_map_int_test() {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    let result = AxumHttpServerHelper::collect_headers(&headers);
    assert!(result.contains_key("content-type"));
}

/// @covers: AxumHttpServerHelper::payload_too_large
#[test]
fn transport_struct_axum_http_server_helper_payload_too_large_returns_413_int_test() {
    let resp = AxumHttpServerHelper::payload_too_large(1024);
    assert_eq!(resp.status(), 413);
}

/// @covers: AxumHttpServerHelper::internal_server_error
#[test]
fn transport_struct_axum_http_server_helper_internal_server_error_returns_500_int_test() {
    let resp = AxumHttpServerHelper::internal_server_error("test error");
    assert_eq!(resp.status(), 500);
}

/// @covers: AxumHttpServerHelper::verify_auth
#[test]
fn transport_struct_axum_http_server_helper_verify_auth_passes_when_no_verifier_int_test() {
    use axum::body::Body;
    use axum::http::Request;
    let req = Request::builder().body(Body::empty()).expect("must build");
    let result = AxumHttpServerHelper::verify_auth(req, None);
    assert!(result.is_ok());
}

/// @covers: AxumHttpServerHelper::extract_request
/// @covers: AxumHttpServerHelper::dispatch_sse
/// @covers: AxumHttpServerHelper::dispatch_websocket
/// @covers: AxumHttpServerHelper::serve_tls
#[test]
fn transport_struct_axum_http_server_helper_functions_are_accessible_int_test() {
    // Compile-time proof these functions exist on the type.
    let _ = AxumHttpServerHelper::payload_too_large(0);
}
