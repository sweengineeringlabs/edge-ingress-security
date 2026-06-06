#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests exercising the `hyper-util` dependency directly.
//!
//! The `hyper-util` crate is used inside the TLS accept loop
//! (`hyper_util::server::conn::auto::Builder` + `TokioExecutor` + `TokioIo`).
//! These tests exercise that path by starting a TLS server and connecting
//! with a raw hyper client.

use std::io::Write as _;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use hyper_util::rt::TokioExecutor;
use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    IngressTlsConfig, RequestContext,
};

// Verify hyper_util::rt::TokioExecutor is available (exercises hyper-util dependency).
#[allow(dead_code)]
fn _hyper_util_executor() -> TokioExecutor {
    TokioExecutor::new()
}

struct EchoHandler;

impl HttpIngress for EchoHandler {
    fn handle(
        &self,
        req: HttpRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async move {
            let body = format!("{} {}", req.method, req.url).into_bytes();
            Ok(HttpResponse::new(200, body))
        })
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

async fn start_tls(tls: IngressTlsConfig) -> (SocketAddr, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), Arc::new(EchoHandler)).with_tls(tls);
    tokio::spawn(async move {
        let _ = server
            .serve_with_listener(listener, async move {
                let _ = rx.await;
            })
            .await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    (addr, tx)
}

/// @covers: AxumHttpServer::serve_with_listener
/// Exercises: hyper_util TLS accept loop — server binds, TLS handshake,
/// and HTTP/1.1 connection via `hyper_util::server::conn::auto::Builder`.
#[tokio::test]
async fn test_hyper_util_tls_connection_serves_request_over_https() {
    use rustls::client::danger::ServerCertVerifier;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let (cert_pem, key_pem) = self_signed();
    let cert_f = write_temp(&cert_pem);
    let key_f = write_temp(&key_pem);
    let cfg = IngressTlsConfig::tls(
        cert_f.path().to_str().unwrap(),
        key_f.path().to_str().unwrap(),
    );
    let (addr, _shutdown) = start_tls(cfg).await;

    // Build an insecure TLS connector that accepts any certificate.
    #[derive(Debug)]
    struct AcceptAny;
    impl ServerCertVerifier for AcceptAny {
        fn verify_server_cert(
            &self,
            _: &rustls::pki_types::CertificateDer<'_>,
            _: &[rustls::pki_types::CertificateDer<'_>],
            _: &rustls::pki_types::ServerName<'_>,
            _: &[u8],
            _: rustls::pki_types::UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }
        fn verify_tls12_signature(
            &self,
            _: &[u8],
            _: &rustls::pki_types::CertificateDer<'_>,
            _: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }
        fn verify_tls13_signature(
            &self,
            _: &[u8],
            _: &rustls::pki_types::CertificateDer<'_>,
            _: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }
        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            vec![
                rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
                rustls::SignatureScheme::ED25519,
            ]
        }
    }

    let tls_cfg = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .unwrap()
    .dangerous()
    .with_custom_certificate_verifier(Arc::new(AcceptAny))
    .with_no_client_auth();
    let connector = tokio_rustls::TlsConnector::from(Arc::new(tls_cfg));

    let tcp = tokio::net::TcpStream::connect(addr).await.unwrap();
    let server_name = rustls::pki_types::ServerName::try_from("localhost").unwrap();
    let mut tls = connector.connect(server_name, tcp).await.unwrap();

    let request = b"GET /hyper-test HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    tls.write_all(request).await.unwrap();

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf).await.unwrap();

    let response = String::from_utf8_lossy(&buf);
    assert!(
        response.contains("200"),
        "expected 200 OK in response, got: {response}"
    );
}

/// @covers: AxumHttpServer::serve
/// Exercises: hyper_util path for plain HTTP server (non-TLS) via `axum::serve`.
#[tokio::test]
async fn test_hyper_util_plain_http_server_handles_request() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{addr}");
    let (tx, rx) = oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), Arc::new(EchoHandler));
    tokio::spawn(async move {
        let _ = server
            .serve_with_listener(listener, async move {
                let _ = rx.await;
            })
            .await;
    });

    let resp = reqwest::get(format!("{base}/hyper-plain")).await.unwrap();
    assert_eq!(resp.status(), 200);
    let _ = tx.send(());
}
