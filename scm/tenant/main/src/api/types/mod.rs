//! Public types for the tenant resolver layer.
pub mod tenant;

pub use tenant::NoopTenantResolver;
pub use tenant::TenantId;
pub use tenant::TenantResolverBox;
pub use tenant::TenantResolverConfig;
pub use tenant::TenantSvc;
