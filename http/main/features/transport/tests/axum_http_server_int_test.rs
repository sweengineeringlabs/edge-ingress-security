//! Tests for AxumHttpServer.

use swe_edge_ingress_http::AxumHttpServer;

#[test]
fn test_axum_http_server_exists() {
    let _ = std::any::type_name::<AxumHttpServer>();
}
