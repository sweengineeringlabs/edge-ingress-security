//! SAF service functions for bearer auth.

use crate::api::traits::{Processor, Validator};
use crate::api::BearerIngressConfig;

/// Return the processor's self-description via the [`Processor`] trait contract.
///
/// This function consumes the `Processor` trait bound, satisfying the SEA
/// requirement that every api/ trait is exercised through the saf facade.
pub fn processor_describe<P: Processor>(p: &P) -> &'static str {
    p.describe()
}

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
