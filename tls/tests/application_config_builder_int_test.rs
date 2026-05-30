//! Contract tests for ApplicationConfigBuilder type alias.

use swe_edge_ingress_tls::TlsSvc;

/// @covers: create_config_builder
#[test]
fn test_application_config_builder_returns_builder_for_this_crate() {
    let builder: swe_edge_configbuilder::ConfigBuilderImpl = TlsSvc::create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
}
