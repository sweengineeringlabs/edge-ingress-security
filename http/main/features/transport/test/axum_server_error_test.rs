//! Tests for AxumServerError.

use swe_edge_ingress_http::AxumServerError;

#[test]
fn test_axum_server_error_exists() {
    let _err = AxumServerError::Serve(std::io::Error::other("test"));
}
