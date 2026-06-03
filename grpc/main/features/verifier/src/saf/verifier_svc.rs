//! SAF service functions for the gRPC verifier.

/// Create a config builder pre-seeded with this crate's package metadata.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
}
