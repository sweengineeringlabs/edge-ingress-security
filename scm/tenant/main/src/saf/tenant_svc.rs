//! SAF impl for [`TenantSvc`] and [`tenant_resolver_from_config`].

use std::sync::Arc;

use crate::api::traits::{TenantResolver, Validator};
use crate::api::types::tenant::NoopTenantResolver;
use crate::api::types::{TenantResolverBox, TenantResolverConfig, TenantSvc};
use crate::spi::header_tenant_resolver::HeaderTenantResolver;
use crate::spi::jwt_claim_tenant_resolver::JwtClaimTenantResolver;
use crate::spi::subdomain_tenant_resolver::SubdomainTenantResolver;

impl TenantSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate a value using its [`Validator`] implementation.
    pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}

/// Build a [`TenantResolverBox`] from `[tenant.resolver]` TOML config.
///
/// | `strategy` value | Resolver wired |
/// |---|---|
/// | `"noop"` (default) | [`NoopTenantResolver`] — always returns `None` |
/// | `"header"` | [`HeaderTenantResolver`] — reads `config.header` HTTP header |
/// | `"subdomain"` | [`SubdomainTenantResolver`] — reads `Host` subdomain |
/// | `"jwt-claim"` | [`JwtClaimTenantResolver`] — reads `config.claim` JWT field |
///
/// Unknown strategy values fall back to [`NoopTenantResolver`].
pub fn tenant_resolver_from_config(config: &TenantResolverConfig) -> TenantResolverBox {
    let inner: Arc<dyn TenantResolver> = match config.strategy.as_str() {
        "header" => Arc::new(HeaderTenantResolver::new(&config.header)),
        "subdomain" => Arc::new(SubdomainTenantResolver),
        "jwt-claim" => Arc::new(JwtClaimTenantResolver::new(&config.claim)),
        _ => Arc::new(NoopTenantResolver),
    };
    TenantResolverBox(inner)
}
