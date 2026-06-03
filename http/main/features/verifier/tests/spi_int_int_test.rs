//! Integration checks for verifier SPI presence.

use swe_edge_ingress_http_verifier::VerifierSvc;

#[test]
fn test_spi_int_config_builder_uses_package_name() {
    let builder = VerifierSvc::create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-http-verifier");
}
