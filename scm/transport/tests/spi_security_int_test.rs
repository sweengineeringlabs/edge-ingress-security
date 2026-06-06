//! Security checks for transport SPI exposure.

use swe_edge_ingress_http::TransportSvc;

#[test]
fn test_spi_security_config_builder_uses_package_name() {
    let builder = TransportSvc::create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-http-transport");
}
