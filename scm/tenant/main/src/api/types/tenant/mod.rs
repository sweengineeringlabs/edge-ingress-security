//! Tenant-domain types.
pub mod noop_tenant_resolver;
pub mod tenant_id;
pub mod tenant_resolver_box;
pub mod tenant_resolver_config;
pub mod tenant_svc;

pub use noop_tenant_resolver::NoopTenantResolver;
pub use tenant_id::TenantId;
pub use tenant_resolver_box::TenantResolverBox;
pub use tenant_resolver_config::TenantResolverConfig;
pub use tenant_svc::TenantSvc;
