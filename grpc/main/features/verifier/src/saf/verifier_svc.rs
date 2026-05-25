//! SAF service functions for the gRPC verifier.

use crate::api::types::ArchitectureConfigBuilder;

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> ArchitectureConfigBuilder {
    ArchitectureConfigBuilder::default()
}
