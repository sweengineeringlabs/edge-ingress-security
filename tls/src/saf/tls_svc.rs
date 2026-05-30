//! TLS SAF — factory method implementations on [`TlsSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::IngressTlsError;
use crate::api::types::IngressTlsConfig;
use crate::api::types::TlsSvc;
use crate::core::DefaultAcceptorBuilder;

impl TlsSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Construct a [`tokio_rustls::TlsAcceptor`] from `config`.
    pub fn build_tls_acceptor(
        config: &IngressTlsConfig,
    ) -> Result<tokio_rustls::TlsAcceptor, IngressTlsError> {
        let acceptor = DefaultAcceptorBuilder::build(config)?;
        Ok(acceptor)
    }
}
