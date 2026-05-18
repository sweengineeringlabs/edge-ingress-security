//! HTTP server declarations and builder methods.

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::port::http_inbound::HttpInbound;

/// Default maximum inbound request body size (4 MiB).
pub const MAX_BODY_BYTES: usize = 4 * 1_024 * 1_024; // 4 MiB

/// Axum-based HTTP server that routes all inbound requests through an
/// [`HttpInbound`] port.
pub struct AxumHttpServer {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn HttpInbound>,
    pub(crate) body_limit: usize,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) bearer_verifier: Option<Arc<dyn TokenVerifier>>,
}

impl AxumHttpServer {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpInbound>) -> Self {
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
    use super::*;
    use crate::api::port::http_health_check::HttpHealthCheck;
    use crate::api::port::http_inbound::HttpInbound;
    use crate::api::port::http_inbound_result::HttpInboundResult;
    use crate::api::value_object::{HttpRequest, HttpResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    struct DummyHandler;
    impl HttpInbound for DummyHandler {
        fn handle(
            &self,
            _: HttpRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
            Box::pin(async { Ok(HttpHealthCheck::healthy()) })
        }
    }

    /// @covers: with_body_limit
    #[test]
    fn test_new_creates_server_with_max_body_bytes_default() {
        let s = AxumHttpServer::new("127.0.0.1:0", Arc::new(DummyHandler));
        assert_eq!(s.body_limit, MAX_BODY_BYTES);
        assert!(s.tls.is_none());
    }

    /// @covers: with_body_limit
    #[test]
    fn test_with_body_limit_overrides_default() {
        let s = AxumHttpServer::new("127.0.0.1:0", Arc::new(DummyHandler)).with_body_limit(1024);
        assert_eq!(s.body_limit, 1024);
    }

    #[test]
    fn test_max_body_bytes_is_4_mib() {
        assert_eq!(MAX_BODY_BYTES, 4 * 1_024 * 1_024);
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_sets_tls_config() {
        let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
        let s = AxumHttpServer::new("127.0.0.1:0", Arc::new(DummyHandler)).with_tls(cfg);
        assert!(s.tls.is_some());
    }

    /// @covers: with_bearer_auth
    #[test]
    fn test_with_bearer_auth_sets_verifier() {
        use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

        struct AlwaysOk;
        impl TokenVerifier for AlwaysOk {
            fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
                Ok(serde_json::from_str("{}").unwrap())
            }
        }

        let s = AxumHttpServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .with_bearer_auth(Arc::new(AlwaysOk));
        assert!(s.bearer_verifier.is_some());
    }
}
