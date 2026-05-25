//! SAF service functions for the gRPC verifier.

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::create_config_builder()
}
