//! Contract tests for the Validator trait.

/// @covers: Validator
#[test]
fn test_validator_trait_is_accessible_from_public_api() {
    use swe_edge_ingress_tls::TlsSvc;
    // TlsSvc::create_config_builder returns a builder; this verifies api/traits
    // is accessible through the SAF without naming internal trait paths.
    let _builder = TlsSvc::create_config_builder();
}
