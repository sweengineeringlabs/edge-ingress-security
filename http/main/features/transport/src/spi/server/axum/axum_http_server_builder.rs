//! Fluent builder for [`AxumHttpServer`].

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use super::axum_http_server::{AxumHttpServer, MAX_BODY_BYTES};
use crate::api::traits::HttpIngress;

/// Fluent builder that constructs an [`AxumHttpServer`].
///
/// Call [`AxumHttpServerBuilder::new`] to start, configure with the fluent
/// setters, and call [`AxumHttpServerBuilder::build`] to get the server.
pub struct AxumHttpServerBuilder {
    bind: String,
    handler: Arc<dyn HttpIngress>,
    body_limit: usize,
    tls: Option<IngressTlsConfig>,
    bearer_verifier: Option<Arc<dyn TokenVerifier>>,
}

impl AxumHttpServerBuilder {
    /// Start a builder for a server that binds to `bind` and delegates to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            body_limit: MAX_BODY_BYTES,
            tls: None,
            bearer_verifier: None,
        }
    }

    /// Override the maximum request body size (default: [`MAX_BODY_BYTES`]).
    pub fn with_body_limit(mut self, limit: usize) -> Self {
        self.body_limit = limit;
        self
    }

    /// Enable TLS (server-side) or mTLS.
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    /// Enable JWT bearer authentication.
    pub fn with_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self {
        self.bearer_verifier = Some(verifier);
        self
    }

    /// Consume the builder and return a configured [`AxumHttpServer`].
    pub fn build(self) -> AxumHttpServer {
        let mut server =
            AxumHttpServer::new(self.bind, self.handler).with_body_limit(self.body_limit);
        if let Some(tls) = self.tls {
            server = server.with_tls(tls);
        }
        if let Some(verifier) = self.bearer_verifier {
            server = server.with_bearer_auth(verifier);
        }
        server
    }
}
