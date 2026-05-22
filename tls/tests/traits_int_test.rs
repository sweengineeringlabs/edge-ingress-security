//! Integration tests covering `api/traits.rs` — verifies the api layer is
//! reachable and that IngressTlsConfig (the primary api value object) behaves
//! correctly through the public facade.

use swe_edge_ingress_tls::{build_tls_acceptor, IngressTlsConfig, IngressTlsError};

/// @covers: api/traits — api layer is accessible; build_tls_acceptor returns
/// IngressTlsError::CertLoad when the certificate path does not exist.
#[test]
fn test_build_tls_acceptor_with_nonexistent_paths_returns_cert_load_error() {
    let cfg = IngressTlsConfig::tls("/nonexistent/cert.pem", "/nonexistent/key.pem");
    let result = build_tls_acceptor(&cfg);
    assert!(
        matches!(result, Err(IngressTlsError::CertLoad(_, _))),
        "expected IngressTlsError::CertLoad for nonexistent cert path"
    );
}

/// @covers: api/traits — IngressTlsConfig::is_mtls distinguishes TLS from mTLS.
#[test]
fn test_ingress_tls_config_is_mtls_reflects_client_ca_presence() {
    let tls = IngressTlsConfig::tls("cert.pem", "key.pem");
    let mtls = IngressTlsConfig::mtls("cert.pem", "key.pem", "ca.pem");
    assert!(!tls.is_mtls());
    assert!(mtls.is_mtls());
}
