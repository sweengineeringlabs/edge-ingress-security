//! Security checks for transport SPI exposure.

use swe_edge_configbuilder::ConfigBuilder;
use swe_edge_ingress_http::create_config_builder;

#[test]
fn test_spi_security_config_builder_uses_package_name() {
    let builder = create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-http-transport");
}
