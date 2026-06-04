//! Struct declaration and constructors for [`MtlsAuthInterceptor`].

use super::MtlsAuthConfig;

/// `GrpcIngressInterceptor` that enforces mTLS-derived identity.
///
/// Rejects any request that arrives without a peer-cert fingerprint in
/// metadata (populated by the TLS layer). Optionally restricts to specific
/// subject CNs or SAN DNS names.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_auth_mtls::{MtlsAuthConfig, MtlsAuthInterceptor};
///
/// // Accept any verified peer.
/// let interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
///
/// // Restrict to specific CNs.
/// let cfg = MtlsAuthConfig::restrict_to_cns(vec!["trusted.internal".to_string()]);
/// let interceptor = MtlsAuthInterceptor::from_config(cfg);
/// ```
#[derive(Debug, Clone)]
pub struct MtlsAuthInterceptor {
    pub(crate) config: MtlsAuthConfig,
}

impl MtlsAuthInterceptor {
    /// Construct from config.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_auth_mtls::{MtlsAuthConfig, MtlsAuthInterceptor};
    /// let cfg = MtlsAuthConfig::restrict_to_cns(vec!["svc.internal".to_string()]);
    /// let _interceptor = MtlsAuthInterceptor::from_config(cfg);
    /// ```
    pub fn from_config(config: MtlsAuthConfig) -> Self {
        Self { config }
    }
    /// Convenience: accept any peer that completed mTLS, no allowlist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_auth_mtls::MtlsAuthInterceptor;
    /// let _interceptor = MtlsAuthInterceptor::allow_any_verified_peer();
    /// ```
    pub fn allow_any_verified_peer() -> Self {
        Self::from_config(MtlsAuthConfig::allow_any_verified_peer())
    }
}
