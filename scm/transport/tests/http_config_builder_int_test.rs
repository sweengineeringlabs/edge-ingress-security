//! Tests for HttpConfigBuilder.

use swe_edge_ingress_http::HttpConfigBuilder;

#[test]
fn test_http_config_builder_new() {
    let builder = HttpConfigBuilder::new();
    let _config = builder.build();
}
