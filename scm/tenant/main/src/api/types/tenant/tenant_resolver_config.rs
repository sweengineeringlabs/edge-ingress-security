//! [`TenantResolverConfig`] — TOML config for the `[tenant.resolver]` section.

use serde::{Deserialize, Serialize};

use crate::api::traits::Validator;

/// Configuration for the `[tenant.resolver]` TOML section.
///
/// # TOML example
///
/// ```toml
/// [tenant.resolver]
/// strategy = "jwt-claim"
/// claim    = "tenant_id"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TenantResolverConfig {
    /// Resolver strategy. One of `"noop"`, `"header"`, `"subdomain"`, `"jwt-claim"`.
    pub strategy: String,
    /// HTTP header name to read for `strategy = "header"` (default: `"X-Tenant-ID"`).
    pub header: String,
    /// JWT claim field to read for `strategy = "jwt-claim"` (default: `"tenant_id"`).
    pub claim: String,
}

impl Default for TenantResolverConfig {
    fn default() -> Self {
        Self {
            strategy: "noop".to_string(),
            header: "X-Tenant-ID".to_string(),
            claim: "tenant_id".to_string(),
        }
    }
}

impl Validator for TenantResolverConfig {
    fn validate(&self) -> Result<(), String> {
        let known = ["noop", "header", "subdomain", "jwt-claim"];
        if known.contains(&self.strategy.as_str()) {
            Ok(())
        } else {
            Err(format!(
                "unknown tenant resolver strategy '{}'; expected one of: {}",
                self.strategy,
                known.join(", ")
            ))
        }
    }
}

impl swe_edge_configbuilder::OptionalSection for TenantResolverConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "tenant.resolver"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "tenant identity resolver strategy (jwt-claim/header/subdomain/noop)",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}
