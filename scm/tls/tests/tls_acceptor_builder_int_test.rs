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
/// @covers: tokio-rustls — TlsAcceptor constructed via TlsSvc::build_tls_acceptor
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
    let result = TlsSvc::build_tls_acceptor(&cfg);
    assert!(result.is_err(), "expected Err for nonexistent PEM files");
    let err = result.err().expect("is_err() verified above");
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

/// @covers: TlsAcceptorBuilder — exercises rustls, rustls-pemfile, tokio-rustls directly
#[test]
fn test_tls_deps_accessible_in_integration_context() {
    use rustls::crypto::ring::default_provider;
    use rustls_pemfile::certs;
    use tokio_rustls::TlsAcceptor;
    // Verify the rustls crypto provider can be created (exercises rustls dep)
    let _provider = default_provider();
    // Verify rustls-pemfile parses empty input gracefully
    let empty: &[u8] = b"";
    let result: Vec<_> = certs(&mut std::io::BufReader::new(empty)).collect();
    assert!(result.is_empty());
    // Verify tokio-rustls TlsAcceptor type is accessible
    let (cert_pem, key_pem) = self_signed();
    let cert_f = write_temp(&cert_pem);
    let key_f = write_temp(&key_pem);
    let cfg = swe_edge_ingress_tls::IngressTlsConfig::tls(
        cert_f.path().to_str().unwrap(),
        key_f.path().to_str().unwrap(),
    );
    let acceptor: TlsAcceptor = swe_edge_ingress_tls::TlsSvc::build_tls_acceptor(&cfg).unwrap();
    drop(acceptor);
}
