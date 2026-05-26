//! Security checks for verifier SPI exposure.

use swe_edge_configbuilder::ConfigBuilder;
use swe_edge_ingress_http_verifier::create_config_builder;

#[test]
fn test_spi_security_config_builder_has_version() {
    let builder = create_config_builder();
    assert!(!builder.version().is_empty());
}
