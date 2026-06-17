//! SAF layer — tenant resolver public facade.
pub(crate) mod tenant_svc;

pub use crate::api::error::TenantError;
pub use crate::api::types::{
    NoopTenantResolver, TenantId, TenantResolverBox, TenantResolverConfig, TenantSvc,
};
pub use tenant_svc::tenant_resolver_from_config;
