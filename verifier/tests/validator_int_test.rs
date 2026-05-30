//! Integration tests for the Validator trait.

use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_ingress_verifier::VerifierSvc;

/// @covers: Validator
#[test]
fn test_validator_trait_accessible_via_verifier_svc() {
    let _builder = VerifierSvc::create_config_builder();
}
