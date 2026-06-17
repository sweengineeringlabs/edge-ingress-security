//! Integration tests — `TenantResolverConfig` + `OptionalSection` + `Validator` impls.
// @covers: api/types/tenant/tenant_resolver_config.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::OptionalSection;
use swe_edge_ingress_tenant::{TenantResolverConfig, TenantSvc};

#[test]
fn test_tenant_resolver_config_section_name_is_tenant_resolver() {
    assert_eq!(TenantResolverConfig::section_name(), "tenant.resolver");
}

#[test]
fn test_tenant_resolver_config_metadata_owner_is_platform_team() {
    let meta = TenantResolverConfig::metadata();
    assert_eq!(meta.owner, "platform-team");
}

#[test]
fn test_tenant_resolver_config_default_strategy_is_noop() {
    let c = TenantResolverConfig::default();
    assert_eq!(c.strategy, "noop");
}

#[test]
fn test_tenant_resolver_config_default_header_is_x_tenant_id() {
    let c = TenantResolverConfig::default();
    assert_eq!(c.header, "X-Tenant-ID");
}

#[test]
fn test_tenant_resolver_config_default_claim_is_tenant_id() {
    let c = TenantResolverConfig::default();
    assert_eq!(c.claim, "tenant_id");
}

#[test]
fn test_tenant_resolver_config_validate_noop_strategy_is_valid() {
    let c = TenantResolverConfig::default();
    assert!(TenantSvc::validate(&c).is_ok());
}

#[test]
fn test_tenant_resolver_config_validate_known_strategies_are_valid() {
    for strategy in ["noop", "header", "subdomain", "jwt-claim"] {
        let c = TenantResolverConfig {
            strategy: strategy.to_string(),
            ..Default::default()
        };
        assert!(
            TenantSvc::validate(&c).is_ok(),
            "expected valid for strategy={strategy}"
        );
    }
}

#[test]
fn test_tenant_resolver_config_validate_unknown_strategy_returns_error() {
    let c = TenantResolverConfig {
        strategy: "oauth2".to_string(),
        ..Default::default()
    };
    assert!(TenantSvc::validate(&c).is_err());
}

#[test]
fn test_tenant_resolver_config_deserializes_strategy_from_json() {
    let json = r#"{"strategy":"header","header":"X-Tenant","claim":"tenant_id"}"#;
    let c: TenantResolverConfig = serde_json::from_str(json).unwrap();
    assert_eq!(c.strategy, "header");
    assert_eq!(c.header, "X-Tenant");
}
