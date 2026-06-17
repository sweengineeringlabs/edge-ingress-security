//! [`NoopTenantResolver`] — default no-op resolver placeholder.

/// Default no-op resolver. Satisfies any [`TenantResolver`](crate::api::traits::TenantResolver)
/// bound without extracting a tenant — always returns `None`.
pub struct NoopTenantResolver;
