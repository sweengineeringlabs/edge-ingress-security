//! Public API tests for verifier SAF facade functions.

use swe_edge_ingress_http_verifier::VerifierSvc;

#[test]
fn test_create_config_builder_returns_builder() {
    let builder = VerifierSvc::create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-http-verifier");
}

#[test]
fn test_create_config_builder_includes_version() {
    let builder = VerifierSvc::create_config_builder();
    assert!(!builder.version().is_empty());
}
