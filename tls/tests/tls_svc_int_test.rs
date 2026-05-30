//! Integration tests for TlsSvc factory methods.

use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_ingress_tls::TlsSvc;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_crate_name() {
    let builder = TlsSvc::create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}
