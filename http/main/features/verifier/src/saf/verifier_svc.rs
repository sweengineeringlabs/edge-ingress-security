//! SAF impl for [`VerifierSvc`].
use crate::api::types::VerifierSvc;

impl VerifierSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate any value implementing the [`Validator`](crate::api::traits::Validator)
    /// contract, returning a human-readable error describing the first failure.
    pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }

    /// Build the crate's primary [`Processor`](crate::api::traits::Processor) —
    /// the verifier processing unit (`service_type = "processor"`).
    pub fn processor() -> impl crate::api::traits::Processor {
        crate::core::processor::verifier_processor::VerifierProcessor
    }
}
