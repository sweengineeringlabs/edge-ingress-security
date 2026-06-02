//! Tests for HttpIngressError.

use swe_edge_ingress_http::HttpIngressError;

#[test]
fn test_http_ingress_error_exists() {
    let _err = HttpIngressError::InvalidInput("test".into());
}
