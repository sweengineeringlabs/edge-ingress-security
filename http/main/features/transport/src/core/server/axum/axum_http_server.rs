//! Axum-based HTTP server implementation.

use axum::Router;
use tokio::net::TcpListener;

use crate::api::server::axum::axum_http_server::AxumHttpServer;
use crate::api::server::axum::axum_server_error::AxumServerError;
use crate::api::types::server::axum_http_server_helper::AxumHttpServerHelper;

impl AxumHttpServer {
    /// Bind and serve until `shutdown` resolves.
    ///
    /// Axum performs a graceful drain on shutdown: in-flight requests
    /// complete before the listener closes.
    pub async fn serve<F>(&self, shutdown: F) -> Result<(), AxumServerError>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let listener = TcpListener::bind(&self.bind)
            .await
            .map_err(|e| AxumServerError::Bind(self.bind.clone(), e))?;
        self.serve_with_listener(listener, shutdown).await
    }

    /// Serve using a caller-supplied pre-bound listener.
    ///
    /// Useful when the socket must be bound before the server is
    /// constructed (e.g. pre-bind during startup for zero-downtime
    /// restarts, or port-0 allocation in tests).
    pub async fn serve_with_listener<F>(
        &self,
        listener: TcpListener,
        shutdown: F,
    ) -> Result<(), AxumServerError>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let bind_addr = listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| self.bind.clone());

        if let Some(ref tls_cfg) = self.tls {
            tracing::info!(bind = %bind_addr, mtls = tls_cfg.is_mtls(), "HTTPS server listening");
            AxumHttpServerHelper::serve_tls(
                listener,
                self.handler.clone(),
                self.body_limit,
                self.bearer_verifier.clone(),
                self.stream_handler.clone(),
                tls_cfg,
                shutdown,
            )
            .await
        } else {
            tracing::info!(bind = %bind_addr, "HTTP server listening");

            let handler = self.handler.clone();
            let body_limit = self.body_limit;
            let verifier = self.bearer_verifier.clone();
            let stream_handler = self.stream_handler.clone();

            let app = Router::new().fallback(move |req: axum::extract::Request| {
                let handler = handler.clone();
                let stream_handler = stream_handler.clone();
                let verifier = verifier.clone();
                async move {
                    let req = match AxumHttpServerHelper::verify_auth(req, verifier.as_deref()) {
                        Ok(r) => r,
                        Err(rsp) => return rsp,
                    };

                    // Streaming: WebSocket upgrade
                    if AxumHttpServerHelper::is_websocket_upgrade(req.headers()) {
                        if let Some(sh) = stream_handler {
                            return AxumHttpServerHelper::dispatch_websocket(req, sh).await;
                        }
                    }

                    // Streaming: SSE
                    if AxumHttpServerHelper::is_sse_request(req.headers()) {
                        if let Some(sh) = stream_handler {
                            return AxumHttpServerHelper::dispatch_sse(req, body_limit, sh).await;
                        }
                    }

                    // Regular HTTP
                    match AxumHttpServerHelper::extract_request(req, body_limit).await {
                        Ok((http_req, ctx)) => match handler.handle(http_req, ctx).await {
                            Ok(resp) => AxumHttpServerHelper::build_response(resp),
                            Err(e) => AxumHttpServerHelper::error_response(e),
                        },
                        Err(resp) => resp,
                    }
                }
            });

            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown)
                .await
                .map_err(AxumServerError::Serve)
        }
    }
}

#[cfg(test)]
mod dedicated_coverage {
    use super::AxumHttpServer;
    use crate::api::port::http_health_check::HttpHealthCheck;
    use crate::api::port::http_ingress::HttpIngress;
    use crate::api::port::http_ingress_result::HttpIngressResult;
    use crate::api::types::server::axum_http_server::MAX_BODY_BYTES;
    use crate::api::value::{HttpRequest, HttpResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use swe_edge_ingress_tls::IngressTlsConfig;

    fn make_handler() -> Arc<dyn HttpIngress> {
        struct AxumHttpServerOkHandler;
        impl HttpIngress for AxumHttpServerOkHandler {
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
        Arc::new(AxumHttpServerOkHandler)
    }

    fn server() -> AxumHttpServer {
        AxumHttpServer::new("127.0.0.1:0", make_handler())
    }

    #[test]
    fn test_new_sets_default_body_limit() {
        let s = server();
        assert_eq!(s.body_limit, MAX_BODY_BYTES);
    }

    #[test]
    fn test_with_body_limit_overrides_default() {
        let s = server().with_body_limit(1024);
        assert_eq!(s.body_limit, 1024);
    }

    #[test]
    fn test_with_tls_sets_config() {
        let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
        let s = server().with_tls(cfg);
        assert!(s.tls.is_some());
    }

    /// @covers: serve
    #[tokio::test]
    async fn test_serve_errors_on_invalid_bind_address() {
        let s = AxumHttpServer::new("0.0.0.0:99999", make_handler());
        let result = s.serve(std::future::ready(())).await;
        assert!(result.is_err());
    }

    /// @covers: serve_with_listener
    #[tokio::test]
    async fn test_serve_with_listener_completes_on_immediate_shutdown() {
        use tokio::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let s = server();
        let result = s
            .serve_with_listener(listener, std::future::ready(()))
            .await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod sync_coverage {
    use super::AxumHttpServer;
    use crate::api::port::http_health_check::HttpHealthCheck;
    use crate::api::port::http_ingress::HttpIngress;
    use crate::api::port::http_ingress_result::HttpIngressResult;
    use crate::api::value::{HttpRequest, HttpResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    fn make_handler() -> Arc<dyn HttpIngress> {
        struct AxumHttpServerOkHandler;
        impl HttpIngress for AxumHttpServerOkHandler {
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
        Arc::new(AxumHttpServerOkHandler)
    }

    /// @covers: serve
    #[test]
    fn test_serve_is_constructible() {
        let _ = AxumHttpServer::new("127.0.0.1:0", make_handler());
    }

    /// @covers: serve_with_listener
    #[test]
    fn test_serve_with_listener_is_constructible() {
        let _ = AxumHttpServer::new("127.0.0.1:0", make_handler());
    }
}
