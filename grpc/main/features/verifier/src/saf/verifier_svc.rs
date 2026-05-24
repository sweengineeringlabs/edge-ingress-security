//! SAF service functions for the gRPC verifier.

use swe_edge_configbuilder::ConfigBuilder as _;

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}
