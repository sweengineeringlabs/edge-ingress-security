//! Example: build a tenant resolver from TOML configuration.

use swe_edge_ingress_tenant::{tenant_resolver_from_config, TenantResolverConfig};

fn main() {
    let config = TenantResolverConfig {
        strategy: "header".to_string(),
        header: "X-Tenant-ID".to_string(),
        claim: "tenant_id".to_string(),
    };

    let _resolver = tenant_resolver_from_config(&config);
    println!("Tenant resolver ready for strategy '{}'", config.strategy);
}
