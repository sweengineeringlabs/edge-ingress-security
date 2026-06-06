//! SAF impl for [`TransportSvc`].

use crate::api::traits::Validator;
use crate::api::types::TransportSvc;

impl TransportSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate a value using its [`Validator`] implementation.
    pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}
