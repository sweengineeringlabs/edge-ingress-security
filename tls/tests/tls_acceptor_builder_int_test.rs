//! Integration tests for TlsSvc::build_tls_acceptor with real PEM files.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::Write as _;
use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError, TlsSvc};

fn self_signed() -> (String, String) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    (cert.cert.pem(), cert.key_pair.serialize_pem())
}

fn write_temp(content: &str) -> tempfile::NamedTempFile {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    f.write_all(content.as_bytes()).unwrap();
    f
}

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_with_valid_tls_config_succeeds() {
    let (cert_pem, key_pem) = self_signed();
    let cert_f = write_temp(&cert_pem);
    let key_f = write_temp(&key_pem);
    let cfg = IngressTlsConfig::tls(
        cert_f.path().to_str().unwrap(),
        key_f.path().to_str().unwrap(),
    );
    assert!(TlsSvc::build_tls_acceptor(&cfg).is_ok());
}

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_with_missing_cert_file_returns_cert_load_error() {
    let cfg = IngressTlsConfig::tls("/nonexistent/cert.pem", "/nonexistent/key.pem");
    let err = TlsSvc::build_tls_acceptor(&cfg).unwrap_err();
    assert!(matches!(err, IngressTlsError::CertLoad(_, _)));
}

/// @covers: TlsSvc::build_tls_acceptor — return type is TlsAcceptor
#[test]
fn test_build_tls_acceptor_returns_tokio_rustls_tls_acceptor_type() {
    let (cert_pem, key_pem) = self_signed();
    let cert_f = write_temp(&cert_pem);
    let key_f = write_temp(&key_pem);
    let cfg = IngressTlsConfig::tls(
        cert_f.path().to_str().unwrap(),
        key_f.path().to_str().unwrap(),
    );
    // Explicitly bind to TlsAcceptor to verify tokio-rustls integration
    let acceptor: swe_edge_ingress_tls::TlsAcceptor = TlsSvc::build_tls_acceptor(&cfg).unwrap();
    drop(acceptor);
}
