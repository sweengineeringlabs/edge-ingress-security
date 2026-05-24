//! SAF service functions for bearer auth.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::traits::Validator;
use crate::api::BearerIngressConfig;

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Returns the authorization metadata key used to carry the extracted bearer subject.
pub fn extracted_bearer_subject_key() -> &'static str {
    crate::api::bearer::metadata_keys::EXTRACTED_BEARER_SUBJECT
}

/// Validates a [`BearerIngressConfig`] against the [`Validator`] contract.
///
/// Returns `Ok(())` when the configuration is valid (non-empty issuer and
/// audience), or an `Err` describing the first violation.
pub fn validate_bearer_config(cfg: &BearerIngressConfig) -> Result<(), String> {
    cfg.validate()
}
