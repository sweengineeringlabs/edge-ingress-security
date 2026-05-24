//! Fluent builder for [`AxumHttpServer`].

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use super::axum_http_server::{AxumHttpServer, MAX_BODY_BYTES};
use crate::api::port::http_ingress::HttpIngress;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::http_health_check::HttpHealthCheck;
    use crate::api::port::http_ingress::HttpIngress;
    use crate::api::port::http_ingress_result::HttpIngressResult;
    use crate::api::value_object::{HttpRequest, HttpResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;

    struct Stub;
    impl HttpIngress for Stub {
        fn handle(
            &self,
            _: HttpRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
            Box::pin(async { Ok(HttpHealthCheck::healthy()) })
        }
    }

    fn handler() -> Arc<dyn HttpIngress> {
        Arc::new(Stub)
    }

    /// @covers: with_body_limit
    #[test]
    fn test_new_creates_builder_with_default_body_limit() {
        let b = AxumHttpServerBuilder::new("127.0.0.1:0", handler());
        let s = b.build();
        assert_eq!(s.body_limit, MAX_BODY_BYTES);
    }

    /// @covers: with_body_limit
    #[test]
    fn test_with_body_limit_overrides_default_body_limit() {
        let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
            .with_body_limit(512)
            .build();
        assert_eq!(s.body_limit, 512);
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_sets_tls_config_on_built_server() {
        let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
        let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
            .with_tls(cfg)
            .build();
        assert!(s.tls.is_some());
    }

    /// @covers: build
    #[test]
    fn test_build_returns_server_with_no_tls_by_default() {
        let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler()).build();
        assert!(s.tls.is_none());
    }

    /// @covers: with_bearer_auth
    #[test]
    fn test_with_bearer_auth_sets_verifier_on_built_server() {
        use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

        struct AlwaysOk;
        impl TokenVerifier for AlwaysOk {
            fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
                Ok(serde_json::from_str("{}").unwrap())
            }
        }

        let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
            .with_bearer_auth(Arc::new(AlwaysOk))
            .build();
        assert!(s.bearer_verifier.is_some());
    }
}
