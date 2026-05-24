//! SAF layer — TLS public facade.

use swe_edge_configbuilder::ConfigBuilder as _;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

pub use crate::api::ingress_tls_error::IngressTlsError;
pub use crate::api::server_config::build_acceptor as build_tls_acceptor;
pub use crate::api::value_object::IngressTlsConfig;

/// Re-export so consumers can name the acceptor type without a direct
/// `tokio-rustls` dependency.
pub use tokio_rustls::TlsAcceptor;
