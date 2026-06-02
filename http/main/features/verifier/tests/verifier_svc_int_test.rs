//! Tests for VerifierSvc.
use swe_edge_ingress_http_verifier::VerifierSvc;
/// @covers: VerifierSvc::create_config_builder
#[test]
fn verifier_struct_verifier_svc_create_config_builder_returns_seeded_builder_int_test() {
    let b = VerifierSvc::create_config_builder();
    assert!(!b.name().is_empty());
}
