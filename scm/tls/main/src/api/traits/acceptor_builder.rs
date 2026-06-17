//! `AcceptorBuilder` — contract for building TLS acceptors.

use crate::api::error::IngressTlsError;
use crate::api::types::IngressTlsConfig;

/// Contract for types that build a TLS acceptor from a config.
pub trait AcceptorBuilder: Send + Sync {
    /// Build a TLS acceptor from `config`.
    fn build_acceptor(
        &self,
        config: &IngressTlsConfig,
    ) -> Result<tokio_rustls::TlsAcceptor, IngressTlsError>;
}
