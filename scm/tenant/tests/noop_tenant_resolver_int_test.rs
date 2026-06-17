//! Integration tests for [`NoopTenantResolver`].
// @covers: api/types/tenant/noop_tenant_resolver.rs
#![allow(clippy::unwrap_used)]

use http::HeaderMap;
use swe_edge_ingress_tenant::{
    tenant_resolver_from_config, NoopTenantResolver, TenantResolverConfig,
};

#[test]
fn test_noop_tenant_resolver_is_publicly_accessible() {
    let _: NoopTenantResolver = NoopTenantResolver;
}

#[test]
fn test_noop_tenant_resolver_via_factory_always_returns_none() {
    let config = TenantResolverConfig {
        strategy: "noop".to_string(),
        ..Default::default()
    };
    let resolver = tenant_resolver_from_config(&config);
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::HeaderName::from_static("x-tenant-id"),
        http::header::HeaderValue::from_static("acme"),
    );
    assert!(resolver.resolve(&headers).is_none());
}
