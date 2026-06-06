//! Tests for HttpServerError.

use swe_edge_ingress_http::HttpServerError;

#[test]
fn test_axum_server_error_exists() {
    let _err = HttpServerError::Serve(std::io::Error::other("test"));
}
