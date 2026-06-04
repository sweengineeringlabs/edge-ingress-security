//! SAF service functions for the gRPC verifier.

/// Create a config builder pre-seeded with this crate's package metadata.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
}

/// Describe a processor implementing the [`Processor`] contract.
pub fn describe_processor<P: crate::api::traits::Processor>(p: &P) -> &'static str {
    p.describe()
}

/// Validate any value implementing the [`Validator`](crate::api::traits::Validator) contract.
pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
    v.validate()
}
