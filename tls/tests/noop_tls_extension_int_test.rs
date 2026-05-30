//! Integration tests for NoopTlsExtension.

use swe_edge_ingress_tls::TlsExtension;
use swe_edge_ingress_tls::{IngressTlsConfig, NoopTlsExtension};

/// @covers: noop_tls_extension
#[test]
fn test_noop_extension_returns_config_unchanged() {
    let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
    let ext = NoopTlsExtension;
    let result = ext.extend(cfg.clone());
    assert_eq!(result.cert_pem_path, cfg.cert_pem_path);
}
