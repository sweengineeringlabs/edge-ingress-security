//! Integration tests exercising the `swe-edge-ingress-tls` dependency directly.
//!
//! Verifies TLS server construction and the `IngressTlsConfig` API as used
//! by `AxumHttpServer::with_tls`.

use std::io::Write as _;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::net::TcpListener;

use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    IngressTlsConfig, RequestContext,
};
use swe_edge_ingress_tls::IngressTlsError;

// Verify swe_edge_ingress_tls is directly accessible (exercises swe-edge-ingress-tls dep).
#[allow(dead_code)]
fn _tls_error_variant() -> Option<IngressTlsError> {
    None
}

struct OkHandler;

impl HttpIngress for OkHandler {
    fn handle(
        &self,
        _req: HttpRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, b"tls-ok".to_vec())) })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

fn self_signed() -> (String, String) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    (cert.cert.pem(), cert.key_pair.serialize_pem())
}

fn write_temp(content: &str) -> tempfile::NamedTempFile {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    f.write_all(content.as_bytes()).unwrap();
    f
}

/// @covers: AxumHttpServer::with_tls
/// Exercises: swe_edge_ingress_tls::IngressTlsConfig::tls construction and
/// the `build_tls_acceptor` called inside `serve_with_listener`.
#[tokio::test]
async fn test_swe_edge_ingress_tls_server_binds_and_starts_without_error() {
    let (cert_pem, key_pem) = self_signed();
    let cert_f = write_temp(&cert_pem);
    let key_f = write_temp(&key_pem);

    let cfg = IngressTlsConfig::tls(
        cert_f.path().to_str().unwrap(),
        key_f.path().to_str().unwrap(),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = AxumHttpServer::new(addr.to_string(), Arc::new(OkHandler)).with_tls(cfg);

    // Start in background and shut down immediately — confirms the TLS acceptor
    // is constructed successfully (would return Err on bad cert/key).
    let result = tokio::spawn(async move {
        server
            .serve_with_listener(listener, std::future::ready(()))
            .await
    })
    .await
    .unwrap();

    assert!(
        result.is_ok(),
        "TLS server startup failed: {:?}",
        result.unwrap_err()
    );
}

/// @covers: AxumHttpServer::with_tls
/// Exercises: swe_edge_ingress_tls error path — missing cert file returns Tls error.
#[tokio::test]
async fn test_swe_edge_ingress_tls_returns_error_for_nonexistent_cert() {
    let cfg = IngressTlsConfig::tls("/no/cert.pem", "/no/key.pem");
    let server = AxumHttpServer::new("127.0.0.1:0", Arc::new(OkHandler)).with_tls(cfg);
    let err = server.serve(std::future::pending::<()>()).await;
    assert!(err.is_err(), "expected TLS error for missing cert");
    let msg = err.unwrap_err().to_string();
    assert!(
        msg.contains("TLS") || msg.contains("cert") || msg.contains("failed"),
        "unexpected error message: {msg}"
    );
}

/// @covers: IngressTlsConfig
/// Exercises: swe_edge_ingress_tls config construction API.
#[test]
fn test_swe_edge_ingress_tls_config_is_not_mtls_for_tls_mode() {
    let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
    assert!(!cfg.is_mtls());
}
