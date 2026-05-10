//! HTTP server declarations and builder methods.

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::port::http_inbound::HttpInbound;

pub const MAX_BODY_BYTES: usize = 4 * 1_024 * 1_024; // 4 MiB

/// Error returned by [`AxumHttpServer::serve`].
#[derive(Debug, thiserror::Error)]
pub enum AxumServerError {
    #[error("failed to bind to {0}: {1}")]
    Bind(String, #[source] std::io::Error),
    #[error("server error: {0}")]
    Serve(#[source] std::io::Error),
    #[error("TLS: {0}")]
    Tls(#[source] swe_edge_ingress_tls::IngressTlsError),
}

/// Axum-based HTTP server that routes all inbound requests through an
/// [`HttpInbound`] port.
pub struct AxumHttpServer {
    pub(crate) bind:             String,
    pub(crate) handler:          Arc<dyn HttpInbound>,
    pub(crate) body_limit:       usize,
    pub(crate) tls:              Option<IngressTlsConfig>,
    pub(crate) bearer_verifier:  Option<Arc<dyn TokenVerifier>>,
}

impl AxumHttpServer {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpInbound>) -> Self {
        Self { bind: bind.into(), handler, body_limit: MAX_BODY_BYTES, tls: None, bearer_verifier: None }
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use edge_domain::RequestContext;
    use crate::api::port::http_inbound::{HttpInbound, HttpInboundResult, HttpHealthCheck};
    use crate::api::value_object::{HttpRequest, HttpResponse};
    use futures::future::BoxFuture;

    struct DummyHandler;
    impl HttpInbound for DummyHandler {
        fn handle(&self, _: HttpRequest, _ctx: RequestContext) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
            Box::pin(async { Ok(HttpHealthCheck::healthy()) })
        }
    }

    /// @covers: AxumHttpServer::new — creates server with MAX_BODY_BYTES default.
    #[test]
    fn test_new_creates_server_with_max_body_bytes_default() {
        let s = AxumHttpServer::new("127.0.0.1:0", Arc::new(DummyHandler));
        assert_eq!(s.body_limit, MAX_BODY_BYTES);
        assert!(s.tls.is_none());
    }

    /// @covers: with_body_limit — overrides the body size cap.
    #[test]
    fn test_with_body_limit_overrides_default() {
        let s = AxumHttpServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .with_body_limit(1024);
        assert_eq!(s.body_limit, 1024);
    }

    /// @covers: MAX_BODY_BYTES — 4 MiB cap.
    #[test]
    fn test_max_body_bytes_is_4_mib() {
        assert_eq!(MAX_BODY_BYTES, 4 * 1_024 * 1_024);
    }

    /// @covers: AxumServerError::Bind — error message contains address.
    #[test]
    fn test_axum_server_error_bind_formats_with_address() {
        let e = AxumServerError::Bind(
            "0.0.0.0:8080".into(),
            std::io::Error::new(std::io::ErrorKind::AddrInUse, "in use"),
        );
        assert!(e.to_string().contains("0.0.0.0:8080"), "{e}");
    }
}
