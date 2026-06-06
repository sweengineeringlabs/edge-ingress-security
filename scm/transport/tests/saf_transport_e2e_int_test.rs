//! Public API tests for transport SAF facade functions.

use swe_edge_ingress_http::TransportSvc;

#[test]
fn test_create_config_builder_returns_builder() {
    let builder = TransportSvc::create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-http-transport");
}

#[test]
fn test_create_config_builder_includes_version() {
    let builder = TransportSvc::create_config_builder();
    assert!(!builder.version().is_empty());
}
