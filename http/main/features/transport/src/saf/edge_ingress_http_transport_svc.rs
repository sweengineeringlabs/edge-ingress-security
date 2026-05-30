//! `validate` SAF facade function for the [`Validator`] trait.

use crate::api::traits::Validator;

/// Return a config builder pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Validate a value using its [`Validator`] implementation.
///
/// Returns `Ok(())` when the value is valid, or a human-readable error message.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}
