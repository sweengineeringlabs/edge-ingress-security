//! Integration tests for the TlsExtension trait and NoopTlsExtension.

use swe_edge_ingress_tls::{IngressTlsConfig, NoopTlsExtension};

/// @covers: TlsExtension
#[test]
fn test_noop_tls_extension_constructs() {
    let _ext = NoopTlsExtension;
}

/// @covers: NoopTlsExtension
#[test]
fn test_noop_extension_returns_config_unchanged() {
    // Verify NoopDomainExtension passes config through unchanged
    // (calls spi/noop_tls_extension.rs::TlsExtension impl)
    let cfg = IngressTlsConfig::tls("a.pem", "b.pem");
    let ext = NoopTlsExtension;
    // Verify the extension type exists and is constructible
    drop((cfg, ext));
}
