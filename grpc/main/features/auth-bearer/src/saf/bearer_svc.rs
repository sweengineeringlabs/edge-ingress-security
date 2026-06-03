//! SAF service functions for bearer auth.

use crate::api::traits::Validator;
use crate::api::BearerIngressConfig;

/// Create a config builder pre-seeded with this crate's package metadata.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
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
