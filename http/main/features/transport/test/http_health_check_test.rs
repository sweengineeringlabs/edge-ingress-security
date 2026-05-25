//! Tests for HttpHealthCheck.

use swe_edge_ingress_http::HttpHealthCheck;

#[test]
fn test_http_health_check_healthy() {
    let check = HttpHealthCheck::healthy();
    assert!(check.healthy);
}
