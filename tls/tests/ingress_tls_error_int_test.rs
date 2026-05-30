//! Integration tests for IngressTlsError display.

use swe_edge_ingress_tls::IngressTlsError;

/// @covers: IngressTlsError::CertLoad
#[test]
fn test_cert_load_error_displays_path() {
    let e = IngressTlsError::CertLoad(
        "server.crt".into(),
        std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
    );
    assert!(e.to_string().contains("server.crt"));
}

/// @covers: IngressTlsError::CertParse
#[test]
fn test_cert_parse_error_displays_reason() {
    let e = IngressTlsError::CertParse("invalid PEM".into());
    assert!(e.to_string().contains("invalid PEM"));
}

/// @covers: IngressTlsError::Config
#[test]
fn test_config_error_displays_message() {
    let e = IngressTlsError::Config("cert/key mismatch".into());
    assert!(e.to_string().contains("cert/key mismatch"));
}
