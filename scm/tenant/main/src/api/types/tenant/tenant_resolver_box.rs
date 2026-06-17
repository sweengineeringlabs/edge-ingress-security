//! [`TenantResolverBox`] — opaque handle to a configured tenant resolver.

use std::sync::Arc;

use http::HeaderMap;

use crate::api::traits::TenantResolver;
use crate::api::types::tenant::TenantId;

/// Opaque handle returned by [`tenant_resolver_from_config`].
///
/// Callers invoke [`resolve`](TenantResolverBox::resolve) directly — the inner
/// [`TenantResolver`] trait is not part of the public surface.
///
/// [`tenant_resolver_from_config`]: crate::tenant_resolver_from_config
pub struct TenantResolverBox(pub(crate) Arc<dyn TenantResolver>);

impl TenantResolverBox {
    /// Attempt to extract a [`TenantId`] from the given request headers.
    ///
    /// Returns `None` for single-tenant deployments or when the tenant signal
    /// is absent (missing header, no JWT, unrecognised subdomain).
    pub fn resolve(&self, headers: &HeaderMap) -> Option<TenantId> {
        self.0.resolve(headers)
    }
}
