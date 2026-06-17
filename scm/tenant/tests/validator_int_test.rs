//! Contract tests for the Validator trait.
// @covers: api/traits/validator.rs

use swe_edge_ingress_tenant::TenantSvc;

/// @covers: Validator
#[test]
fn test_validator_trait_is_accessible_through_saf() {
    // TenantSvc::validate accepts any type implementing Validator.
    // TenantResolverConfig implements Validator — this confirms the trait is
    // reachable via the SAF without naming internal trait paths.
    use swe_edge_ingress_tenant::TenantResolverConfig;
    let config = TenantResolverConfig::default();
    assert!(TenantSvc::validate(&config).is_ok());
}
