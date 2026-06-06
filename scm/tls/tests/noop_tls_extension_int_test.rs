//! Integration tests for NoopTlsExtension.

use swe_edge_ingress_tls::NoopTlsExtension;

/// @covers: noop_tls_extension
#[test]
fn tls_struct_noop_tls_extension_is_publicly_exported_int_test() {
    let _ = std::any::type_name::<NoopTlsExtension>();
}
