//! Integration tests for `NoopVerifierExtension` — the SPI no-op placeholder.

use swe_edge_ingress_verifier::NoopVerifierExtension;

/// @covers: NoopVerifierExtension
#[test]
fn verifier_struct_noop_verifier_extension_constructs_via_new_int_test() {
    let _ext = NoopVerifierExtension::new();
}
