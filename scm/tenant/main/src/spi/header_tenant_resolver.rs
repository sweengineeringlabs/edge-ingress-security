//! [`HeaderTenantResolver`] — extracts tenant ID from a named HTTP request header.

use http::HeaderMap;

use crate::api::traits::TenantResolver;
use crate::api::types::TenantId;

/// Reads a configurable HTTP header (default `X-Tenant-ID`) and wraps its value
/// as a [`TenantId`].
///
/// Returns `None` when the header is absent or its value is not valid UTF-8.
#[derive(Debug)]
pub(crate) struct HeaderTenantResolver {
    header_name: String,
}

impl HeaderTenantResolver {
    pub(crate) fn new(header_name: impl Into<String>) -> Self {
        Self {
            header_name: header_name.into(),
        }
    }
}

impl TenantResolver for HeaderTenantResolver {
    fn resolve(&self, headers: &HeaderMap) -> Option<TenantId> {
        let value = headers.get(&self.header_name)?.to_str().ok()?;
        if value.is_empty() {
            return None;
        }
        Some(TenantId::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers_with(name: &'static str, value: &'static str) -> HeaderMap {
        let mut m = HeaderMap::new();
        m.insert(
            http::header::HeaderName::from_static(name),
            http::header::HeaderValue::from_static(value),
        );
        m
    }

    #[test]
    fn test_header_tenant_resolver_new_stores_header_name() {
        let r = HeaderTenantResolver::new("x-custom-header");
        assert_eq!(r.header_name, "x-custom-header");
    }

    #[test]
    fn test_header_tenant_resolver_resolve_present_header_returns_tenant_id() {
        let r = HeaderTenantResolver::new("x-tenant-id");
        let h = headers_with("x-tenant-id", "acme");
        assert_eq!(r.resolve(&h).unwrap().as_str(), "acme");
    }

    #[test]
    fn test_header_tenant_resolver_resolve_absent_header_returns_none() {
        let r = HeaderTenantResolver::new("x-tenant-id");
        assert!(r.resolve(&HeaderMap::new()).is_none());
    }

    #[test]
    fn test_header_tenant_resolver_resolve_empty_value_returns_none() {
        let r = HeaderTenantResolver::new("x-tenant-id");
        let h = headers_with("x-tenant-id", "");
        assert!(r.resolve(&h).is_none());
    }

    #[test]
    fn test_header_tenant_resolver_resolve_custom_header_name_returns_tenant_id() {
        let r = HeaderTenantResolver::new("x-org");
        let h = headers_with("x-org", "globex");
        assert_eq!(r.resolve(&h).unwrap().as_str(), "globex");
    }
}
