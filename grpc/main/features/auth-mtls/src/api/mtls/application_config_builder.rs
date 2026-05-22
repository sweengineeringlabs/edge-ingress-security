//! Builder for application-level mTLS configuration.

use super::mtls_auth_config::MtlsAuthConfig;

/// Fluent builder for [`MtlsAuthConfig`] loaded from `config/application.toml`.
#[derive(Debug, Default)]
pub struct ApplicationConfigBuilder {
    config: MtlsAuthConfig,
}

impl ApplicationConfigBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether a peer certificate is required.
    pub fn require_peer_cert(mut self, require: bool) -> Self {
        if !require {
            self.config.allow_unauthenticated_methods = true;
        }
        self
    }

    /// Set the allowed CNs allowlist.
    pub fn allowed_cns(mut self, cns: Vec<String>) -> Self {
        self.config.allowed_cns = cns;
        self
    }

    /// Build the [`MtlsAuthConfig`].
    pub fn build(self) -> MtlsAuthConfig {
        self.config
    }
}
