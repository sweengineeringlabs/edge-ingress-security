//! Integration tests for VerifierSvc factory methods.

use swe_edge_ingress_verifier::VerifierSvc;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_seeded_with_crate_name() {
    let builder = VerifierSvc::create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
}
