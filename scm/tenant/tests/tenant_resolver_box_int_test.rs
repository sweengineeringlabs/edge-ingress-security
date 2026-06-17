//! Integration tests for [`TenantResolverBox`].
// @covers: api/types/tenant/tenant_resolver_box.rs
#![allow(clippy::unwrap_used)]

use http::HeaderMap;
use swe_edge_ingress_tenant::{tenant_resolver_from_config, TenantResolverConfig};

#[test]
fn test_tenant_resolver_box_resolve_delegates_to_inner_resolver() {
    let config = TenantResolverConfig {
        strategy: "header".to_string(),
        header: "x-tenant-id".to_string(),
        ..Default::default()
    };
    let resolver_box = tenant_resolver_from_config(&config);
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::HeaderName::from_static("x-tenant-id"),
        http::header::HeaderValue::from_static("acme"),
    );
    assert_eq!(resolver_box.resolve(&headers).unwrap().as_str(), "acme");
}

#[test]
fn test_tenant_resolver_box_resolve_returns_none_when_no_match() {
    let config = TenantResolverConfig::default();
    let resolver_box = tenant_resolver_from_config(&config);
    assert!(resolver_box.resolve(&HeaderMap::new()).is_none());
}
