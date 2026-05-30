//! `TlsExtension` — extension hook for downstream TLS customisation.

use crate::api::types::IngressTlsConfig;

/// Downstream extension point for TLS configuration.
///
/// Implement to intercept and transform a [`IngressTlsConfig`] before
/// the acceptor is built.
pub trait TlsExtension: Send + Sync {
    /// Apply extension logic to `config`; return the (possibly modified)
    /// config.
    fn extend(&self, config: IngressTlsConfig) -> IngressTlsConfig;
}
