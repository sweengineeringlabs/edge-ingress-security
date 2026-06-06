//! Integration tests for IngressTlsConfig constructors.

use swe_edge_ingress_tls::IngressTlsConfig;

/// @covers: tls
#[test]
fn test_tls_sets_cert_and_key_no_client_ca() {
    let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
    assert_eq!(cfg.cert_pem_path, "cert.pem");
    assert_eq!(cfg.key_pem_path, "key.pem");
    assert!(cfg.client_ca_pem_path.is_none());
}

/// @covers: mtls
#[test]
fn test_mtls_sets_client_ca() {
    let cfg = IngressTlsConfig::mtls("cert.pem", "key.pem", "ca.pem");
    assert_eq!(cfg.client_ca_pem_path.as_deref(), Some("ca.pem"));
}

/// @covers: is_mtls
#[test]
fn test_is_mtls_reflects_presence_of_client_ca() {
    assert!(IngressTlsConfig::mtls("c", "k", "ca").is_mtls());
    assert!(!IngressTlsConfig::tls("c", "k").is_mtls());
}
