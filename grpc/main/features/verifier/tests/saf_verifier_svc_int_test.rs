//! Public API tests for verifier SAF layer.

use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_ingress_grpc_verifier::create_config_builder;

#[test]
fn test_create_config_builder_returns_builder_with_name_and_version() {
    let builder = create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}
