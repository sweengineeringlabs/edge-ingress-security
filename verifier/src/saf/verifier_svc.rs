//! Verifier SAF — factory methods on [`VerifierSvc`].

use swe_edge_configbuilder::{ConfigBuilder as _, ConfigBuilderImpl, ConfigLoaderFactory};

use crate::api::types::VerifierSvc;

impl VerifierSvc {
    /// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }
}
