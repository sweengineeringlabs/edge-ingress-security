//! SAF wrapper for the Validator trait.

use crate::api::traits::Validator;
pub use swe_edge_configbuilder::create_config_builder;

/// Validate any value that implements the [`Validator`] trait.
///
/// Returns `Ok(())` when the value passes all validation checks, or a
/// human-readable `Err(String)` describing the first failure.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}
