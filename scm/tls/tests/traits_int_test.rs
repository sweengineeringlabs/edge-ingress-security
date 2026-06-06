//! Integration tests for IngressTlsConfig and TlsSvc::build_tls_acceptor.

use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError, TlsSvc};

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_with_nonexistent_paths_returns_cert_load_error() {
    let cfg = IngressTlsConfig::tls("/nonexistent/cert.pem", "/nonexistent/key.pem");
    let result = TlsSvc::build_tls_acceptor(&cfg);
    assert!(
        matches!(result, Err(IngressTlsError::CertLoad(_, _))),
        "expected IngressTlsError::CertLoad for nonexistent cert path"
    );
}

/// @covers: IngressTlsConfig::is_mtls
#[test]
fn test_ingress_tls_config_is_mtls_reflects_client_ca_presence() {
    let tls = IngressTlsConfig::tls("cert.pem", "key.pem");
    let mtls = IngressTlsConfig::mtls("cert.pem", "key.pem", "ca.pem");
    assert!(!tls.is_mtls());
    assert!(mtls.is_mtls());
}
