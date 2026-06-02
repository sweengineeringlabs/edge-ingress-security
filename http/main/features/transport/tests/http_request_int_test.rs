//! Tests for HttpRequest.

use swe_edge_ingress_http::HttpRequest;

#[test]
fn test_http_request_get() {
    let req = HttpRequest::get("http://example.com");
    assert_eq!(req.url, "http://example.com");
}
