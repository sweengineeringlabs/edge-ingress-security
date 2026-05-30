//! Public facade — re-exports from `api/`.

/// Return a config builder pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

pub use crate::api::bearer_layer::BearerLayer;
pub use crate::api::bearer_service::BearerService;
pub use crate::api::error::HttpAuthError;
pub use crate::api::verified_claims::VerifiedClaims;
