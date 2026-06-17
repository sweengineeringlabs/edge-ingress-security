//! [`SubdomainTenantResolver`] — extracts tenant ID from the first subdomain label.

use http::HeaderMap;

use crate::api::traits::TenantResolver;
use crate::api::types::TenantId;

/// Reads the `Host` header and uses the first dot-separated label as the tenant ID.
///
/// Example: `acme.example.com` → `TenantId("acme")`.
///
/// Returns `None` when the `Host` header is absent, not valid UTF-8, has no
/// subdomain (e.g. bare `example.com`), or the first label is empty.
#[derive(Debug, Default)]
pub(crate) struct SubdomainTenantResolver;

impl TenantResolver for SubdomainTenantResolver {
    fn resolve(&self, headers: &HeaderMap) -> Option<TenantId> {
        let host = headers.get(http::header::HOST)?.to_str().ok()?;
        // Strip optional port suffix (host:port).
        let host = host.split(':').next().unwrap_or(host);
        // A host with fewer than 3 dot-separated labels has no subdomain.
        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() < 3 || parts[0].is_empty() {
            return None;
        }
        Some(TenantId::new(parts[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host_headers(host: &'static str) -> HeaderMap {
        let mut m = HeaderMap::new();
        m.insert(
            http::header::HOST,
            http::header::HeaderValue::from_static(host),
        );
        m
    }

    #[test]
    fn test_subdomain_tenant_resolver_resolve_three_part_host_returns_subdomain() {
        let r = SubdomainTenantResolver;
        assert_eq!(
            r.resolve(&host_headers("acme.example.com"))
                .unwrap()
                .as_str(),
            "acme"
        );
    }

    #[test]
    fn test_subdomain_tenant_resolver_resolve_host_with_port_strips_port() {
        let r = SubdomainTenantResolver;
        assert_eq!(
            r.resolve(&host_headers("acme.example.com:8080"))
                .unwrap()
                .as_str(),
            "acme"
        );
    }

    #[test]
    fn test_subdomain_tenant_resolver_resolve_bare_host_returns_none() {
        let r = SubdomainTenantResolver;
        // "example.com" → first label is "example", but no third label → None
        assert!(r.resolve(&host_headers("example.com")).is_none());
    }

    #[test]
    fn test_subdomain_tenant_resolver_resolve_no_host_header_returns_none() {
        let r = SubdomainTenantResolver;
        assert!(r.resolve(&HeaderMap::new()).is_none());
    }
}
