//! Integration tests — `TenantResolver` implementations via SAF factory.
// @covers: api/traits/tenant_resolver.rs
// @covers: api/types/tenant/noop_tenant_resolver.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use http::{HeaderMap, HeaderValue};
use swe_edge_ingress_tenant::{tenant_resolver_from_config, TenantResolverConfig};

fn make_jwt(payload_json: &str) -> String {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none"}"#);
    let payload = URL_SAFE_NO_PAD.encode(payload_json);
    format!("{header}.{payload}.sig")
}

fn bearer_headers(token: &str) -> HeaderMap {
    let mut m = HeaderMap::new();
    m.insert(
        http::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );
    m
}

fn header_map_with(name: &'static str, value: &'static str) -> HeaderMap {
    let mut m = HeaderMap::new();
    m.insert(
        http::header::HeaderName::from_static(name),
        HeaderValue::from_static(value),
    );
    m
}

// --- factory smoke ---

/// @covers: tenant_resolver_from_config
#[test]
fn test_tenant_resolver_from_config_returns_resolver_box_for_default_config() {
    let config = TenantResolverConfig::default();
    let _r = tenant_resolver_from_config(&config);
}

// --- noop strategy ---

#[test]
fn test_tenant_resolver_noop_strategy_always_returns_none() {
    let config = TenantResolverConfig::default();
    let r = tenant_resolver_from_config(&config);
    assert!(r.resolve(&HeaderMap::new()).is_none());
}

// --- header strategy ---

#[test]
fn test_tenant_resolver_header_strategy_returns_header_value_as_tenant_id() {
    let config = TenantResolverConfig {
        strategy: "header".to_string(),
        header: "x-tenant-id".to_string(),
        claim: "tenant_id".to_string(),
    };
    let r = tenant_resolver_from_config(&config);
    let h = header_map_with("x-tenant-id", "acme");
    assert_eq!(r.resolve(&h).unwrap().as_str(), "acme");
}

#[test]
fn test_tenant_resolver_header_strategy_missing_header_returns_none() {
    let config = TenantResolverConfig {
        strategy: "header".to_string(),
        ..Default::default()
    };
    let r = tenant_resolver_from_config(&config);
    assert!(r.resolve(&HeaderMap::new()).is_none());
}

// --- subdomain strategy ---

#[test]
fn test_tenant_resolver_subdomain_strategy_returns_first_label_as_tenant_id() {
    let config = TenantResolverConfig {
        strategy: "subdomain".to_string(),
        ..Default::default()
    };
    let r = tenant_resolver_from_config(&config);
    let h = header_map_with("host", "acme.example.com");
    assert_eq!(r.resolve(&h).unwrap().as_str(), "acme");
}

#[test]
fn test_tenant_resolver_subdomain_strategy_bare_host_returns_none() {
    let config = TenantResolverConfig {
        strategy: "subdomain".to_string(),
        ..Default::default()
    };
    let r = tenant_resolver_from_config(&config);
    let h = header_map_with("host", "example.com");
    assert!(r.resolve(&h).is_none());
}

// --- jwt-claim strategy ---

#[test]
fn test_tenant_resolver_jwt_claim_strategy_returns_claim_value_as_tenant_id() {
    let config = TenantResolverConfig {
        strategy: "jwt-claim".to_string(),
        claim: "tenant_id".to_string(),
        ..Default::default()
    };
    let r = tenant_resolver_from_config(&config);
    let jwt = make_jwt(r#"{"sub":"user1","tenant_id":"globex"}"#);
    assert_eq!(r.resolve(&bearer_headers(&jwt)).unwrap().as_str(), "globex");
}

#[test]
fn test_tenant_resolver_jwt_claim_strategy_missing_claim_returns_none() {
    let config = TenantResolverConfig {
        strategy: "jwt-claim".to_string(),
        claim: "tenant_id".to_string(),
        ..Default::default()
    };
    let r = tenant_resolver_from_config(&config);
    let jwt = make_jwt(r#"{"sub":"user1"}"#);
    assert!(r.resolve(&bearer_headers(&jwt)).is_none());
}

// --- unknown strategy falls back to noop ---

#[test]
fn test_tenant_resolver_unknown_strategy_falls_back_to_noop() {
    let config = TenantResolverConfig {
        strategy: "custom".to_string(),
        ..Default::default()
    };
    let r = tenant_resolver_from_config(&config);
    assert!(r.resolve(&HeaderMap::new()).is_none());
}
