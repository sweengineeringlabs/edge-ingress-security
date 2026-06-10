//! [`TenantResolver`] — primary trait for extracting a tenant identity from inbound headers.

use http::HeaderMap;

use crate::api::types::TenantId;

/// Extracts a [`TenantId`] from inbound HTTP request headers.
///
/// Implementations are strategy objects — the active impl is chosen via
/// [`TenantResolverConfig::strategy`] and wired by [`tenant_resolver_from_config`].
///
/// [`TenantResolverConfig::strategy`]: crate::TenantResolverConfig::strategy
/// [`tenant_resolver_from_config`]: crate::tenant_resolver_from_config
pub trait TenantResolver: Send + Sync {
    /// Attempt to extract a [`TenantId`] from the given request headers.
    ///
    /// Returns `None` for single-tenant deployments or when the tenant signal
    /// is absent (e.g. missing header, no JWT, unrecognised subdomain).
    fn resolve(&self, headers: &HeaderMap) -> Option<TenantId>;
}
