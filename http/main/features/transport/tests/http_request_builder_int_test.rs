//! Tests for HttpRequestBuilder.

use swe_edge_ingress_http::HttpRequestBuilder;

#[test]
fn test_http_request_builder_get() {
    let builder = HttpRequestBuilder::get("http://example.com");
    let _req = builder.build();
}
