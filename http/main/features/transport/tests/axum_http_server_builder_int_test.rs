//! Tests for AxumHttpServerBuilder.

use swe_edge_ingress_http::AxumHttpServerBuilder;

#[test]
fn test_axum_http_server_builder_exists() {
    let _ = std::any::type_name::<AxumHttpServerBuilder>();
}
