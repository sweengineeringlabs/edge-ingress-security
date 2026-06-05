//! HTTP server declarations and builder methods.

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::traits::HttpIngress;
use crate::api::traits::HttpStream;

/// Default maximum inbound request body size (4 MiB).
pub const MAX_BODY_BYTES: usize = 4 * 1_024 * 1_024; // 4 MiB

/// Axum-based HTTP server that routes all inbound requests through an
/// [`HttpIngress`] port.
pub struct AxumHttpServer {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn HttpIngress>,
    pub(crate) body_limit: usize,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) bearer_verifier: Option<Arc<dyn TokenVerifier>>,
    pub(crate) stream_handler: Option<Arc<dyn HttpStream>>,
}

impl AxumHttpServer {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            body_limit: MAX_BODY_BYTES,
            tls: None,
            bearer_verifier: None,
            stream_handler: None,
        }
    }

    /// Attach an [`HttpStream`] handler for SSE and WebSocket requests.
    ///
    /// When set, `Accept: text/event-stream` requests route to
    /// [`HttpStream::handle_sse`] and `Upgrade: websocket` requests
    /// route to [`HttpStream::handle_websocket`].
    pub fn with_stream_handler(mut self, handler: Arc<dyn HttpStream>) -> Self {
        self.stream_handler = Some(handler);
        self
    }

    /// Override the maximum request body size (default: [`MAX_BODY_BYTES`]).
    pub fn with_body_limit(mut self, limit: usize) -> Self {
        self.body_limit = limit;
        self
    }

    /// Enable TLS (server-side) or mTLS.
    ///
    /// Use [`IngressTlsConfig::tls`] for one-way TLS and
    /// [`IngressTlsConfig::mtls`] to require a client certificate.
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    /// Enable JWT bearer authentication.
    ///
    /// Requests without a valid `Authorization: Bearer <token>` header receive
    /// a `401 Unauthorized` response.  Valid tokens produce an authenticated
    /// [`RequestContext`](edge_domain::RequestContext) that flows to handlers.
    pub fn with_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self {
        self.bearer_verifier = Some(verifier);
        self
    }
}
