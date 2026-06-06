//! Tests for HttpResponse.

use swe_edge_ingress_http::HttpResponse;

#[test]
fn test_http_response_new() {
    let resp = HttpResponse::new(200, b"ok".to_vec());
    assert_eq!(resp.status, 200);
    assert!(resp.is_success());
}
