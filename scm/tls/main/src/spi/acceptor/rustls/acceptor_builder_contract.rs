//! `AcceptorBuilder` — rustls-backed contract for building TLS acceptors.
//!
//! The acceptor product is a [`tokio_rustls::TlsAcceptor`], so this contract is
//! technology-bound and lives in `spi/` rather than the neutral `api/` layer
//! (ADR-008 §1).

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
