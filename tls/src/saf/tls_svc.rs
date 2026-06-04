//! TLS SAF — factory method implementations on [`TlsSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::ingress_tls_error::IngressTlsError;
use crate::api::types::ingress_tls_config::IngressTlsConfig;
use crate::api::types::TlsSvc;
use crate::core::DefaultAcceptorBuilder;

impl TlsSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_tls::TlsSvc;
    ///
    /// let loader = TlsSvc::create_config_builder()
    ///     .with_config_dir("config/")
    ///     .build_loader()
    ///     .expect("config dir accessible");
    /// ```
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Construct a [`tokio_rustls::TlsAcceptor`] from `config`.
    ///
    /// Reads both PEM files eagerly — failures surface here at startup, not at
    /// connection time. The acceptor is backed by the `ring` CryptoProvider
    /// without installing a process-wide default.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_tls::{IngressTlsConfig, TlsSvc};
    ///
    /// let config = IngressTlsConfig::tls("certs/server.crt", "certs/server.key");
    /// let acceptor = TlsSvc::build_tls_acceptor(&config)
    ///     .expect("PEM files must exist and be valid");
    /// // pass acceptor to TonicGrpcServer or AxumHttpServer .with_tls()
    /// ```
    pub fn build_tls_acceptor(
        config: &IngressTlsConfig,
    ) -> Result<tokio_rustls::TlsAcceptor, IngressTlsError> {
        let acceptor = DefaultAcceptorBuilder::build(config)?;
        Ok(acceptor)
    }

    /// Validate any value implementing the [`Validator`](crate::api::traits::validator::Validator)
    /// contract, returning a human-readable error describing the first failure.
    pub fn validate<V: crate::api::traits::validator::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}
