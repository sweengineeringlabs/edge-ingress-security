//! No-op [`TenantResolver`] impl for [`NoopTenantResolver`].

use http::HeaderMap;

use crate::api::traits::TenantResolver;
use crate::api::types::tenant::NoopTenantResolver;
use crate::api::types::TenantId;

impl TenantResolver for NoopTenantResolver {
    fn resolve(&self, _headers: &HeaderMap) -> Option<TenantId> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_tenant_resolver_resolve_always_returns_none() {
        let r = NoopTenantResolver;
        assert!(r.resolve(&HeaderMap::new()).is_none());
    }

    #[test]
    fn test_noop_tenant_resolver_resolve_ignores_any_headers() {
        let r = NoopTenantResolver;
        let mut m = HeaderMap::new();
        m.insert(
            http::header::HeaderName::from_static("x-tenant-id"),
            http::header::HeaderValue::from_static("acme"),
        );
        assert!(r.resolve(&m).is_none());
    }
}
