//! `spi` — external-lib-backed tenant resolver adapters.
pub(crate) mod header_tenant_resolver;
pub(crate) mod jwt_claim_tenant_resolver;
pub(crate) mod noop_tenant_resolver;
pub(crate) mod subdomain_tenant_resolver;
