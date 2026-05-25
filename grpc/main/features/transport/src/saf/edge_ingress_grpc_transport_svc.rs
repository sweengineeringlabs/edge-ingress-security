//! SAF wrapper for the Validator trait.

use crate::api::application::ApplicationConfigBuilder;
use crate::api::traits::Validator;

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> ApplicationConfigBuilder {
    ApplicationConfigBuilder::default()
}

/// Validate any value that implements the [`Validator`] trait.
///
/// Returns `Ok(())` when the value passes all validation checks, or a
/// human-readable `Err(String)` describing the first failure.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}
