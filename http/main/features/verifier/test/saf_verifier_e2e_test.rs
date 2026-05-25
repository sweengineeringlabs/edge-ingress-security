//! Public API tests for verifier SAF facade functions.

use swe_edge_configbuilder::ConfigBuilder;
use swe_edge_ingress_http_verifier::create_config_builder;

#[test]
fn test_create_config_builder_returns_builder() {
    let builder = create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-http-verifier");
}

#[test]
fn test_create_config_builder_includes_version() {
    let builder = create_config_builder();
    assert!(!builder.version().is_empty());
}
