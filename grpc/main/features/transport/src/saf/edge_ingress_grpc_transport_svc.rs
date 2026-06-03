//! SAF wrapper for the Validator trait.

use crate::api::traits::Validator;

/// Create a config builder pre-seeded with this crate's package metadata.
///
/// Thin wrapper over [`swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder`]
/// (the free `create_config_builder` function was replaced by a factory method).
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
}

/// Validate any value that implements the [`Validator`] trait.
///
/// Returns `Ok(())` when the value passes all validation checks, or a
/// human-readable `Err(String)` describing the first failure.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}
