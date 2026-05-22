//! Struct declaration and constructors for [`MtlsAuthInterceptor`].

use crate::api::mtls::MtlsAuthConfig;

/// `GrpcIngressInterceptor` that enforces mTLS-derived identity.
#[derive(Debug, Clone)]
pub struct MtlsAuthInterceptor {
    pub(crate) config: MtlsAuthConfig,
}

impl MtlsAuthInterceptor {
    /// Construct from config.
    pub fn from_config(config: MtlsAuthConfig) -> Self {
        Self { config }
    }
    /// Convenience: accept any peer that completed mTLS, no allowlist.
    pub fn allow_any_verified_peer() -> Self {
        Self::from_config(MtlsAuthConfig::allow_any_verified_peer())
    }
}
