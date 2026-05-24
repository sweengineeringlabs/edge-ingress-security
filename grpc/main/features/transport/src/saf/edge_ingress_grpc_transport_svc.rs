//! SAF wrapper for the Validator trait.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::traits::Validator;

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Validate any value that implements the [`Validator`] trait.
///
/// Returns `Ok(())` when the value passes all validation checks, or a
/// human-readable `Err(String)` describing the first failure.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::GrpcServerConfig;

    /// @covers: validate
    #[test]
    fn test_validate_returns_ok_for_plaintext_config() {
        let cfg = GrpcServerConfig::default().allow_plaintext();
        assert!(validate(&cfg).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_returns_err_for_tls_required_without_tls_config() {
        let cfg = GrpcServerConfig::default();
        assert!(validate(&cfg).is_err());
    }
}
