//! SAF factory functions for the gRPC transport crate.

use crate::api::traits::Validator;

/// Return a config builder pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
    b = b.with_name(env!("CARGO_PKG_NAME"));
    b = b.with_version(env!("CARGO_PKG_VERSION"));
    b
}

/// Validate any value that implements the [`Validator`] trait.
///
/// Returns `Ok(())` when the value passes all validation checks, or a
/// human-readable `Err(String)` describing the first failure.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}
