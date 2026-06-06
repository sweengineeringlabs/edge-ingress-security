//! Tests for HttpHandlerAdapter.

use swe_edge_ingress_http::HttpHandlerAdapter;

#[test]
fn test_http_handler_adapter_exists() {
    // Type is generic and requires construction with parameters.
    // This test verifies the type is accessible in the public API.
    let _ = std::any::type_name::<HttpHandlerAdapter<(), ()>>();
}
