//! `TlsExtension` — extension hook for downstream TLS customisation.

use crate::api::vo::IngressTlsConfig;

/// Downstream extension point for TLS configuration.
///
/// Implement to intercept and transform a [`IngressTlsConfig`] before
/// the acceptor is built.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "SEA api/ interface anchor — spi implements it; no public caller in production"
    )
)]
pub trait TlsExtension: Send + Sync {
    /// Apply extension logic to `config`; return the (possibly modified)
    /// config.
    fn extend(&self, config: IngressTlsConfig) -> IngressTlsConfig;
}
