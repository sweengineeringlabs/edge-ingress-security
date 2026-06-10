//! Integration tests for [`TenantSvc`] factory methods.
// @covers: api/types/tenant/tenant_svc.rs
#![allow(clippy::unwrap_used)]

use swe_edge_ingress_tenant::{TenantResolverConfig, TenantSvc};

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_crate_name() {
    let builder = TenantSvc::create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: validate
#[test]
fn test_validate_valid_config_returns_ok() {
    let config = TenantResolverConfig::default();
    assert!(TenantSvc::validate(&config).is_ok());
}

/// @covers: validate
#[test]
fn test_validate_unknown_strategy_returns_err() {
    let config = TenantResolverConfig {
        strategy: "magic".to_string(),
        ..Default::default()
    };
    let result = TenantSvc::validate(&config);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("magic"));
}
