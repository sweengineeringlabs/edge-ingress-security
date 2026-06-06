//! Tests for HttpConfigValidator.

use swe_edge_ingress_http::HttpConfigValidator;

#[test]
fn test_http_config_validator_exists() {
    let _ = std::any::type_name::<HttpConfigValidator>();
}
