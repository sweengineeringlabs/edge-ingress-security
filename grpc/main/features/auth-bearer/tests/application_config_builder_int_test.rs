//! Integration tests for [`create_config_builder`].

use swe_edge_ingress_grpc_auth_bearer::create_config_builder;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name() {
    let b = create_config_builder();
    assert_eq!(b.name(), "swe-edge-configbuilder");
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_version_matches_package_version() {
    let b = create_config_builder();
    assert_eq!(b.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_with_name_overrides_preset() {
    let b = create_config_builder().with_name("override-name");
    assert_eq!(b.name(), "override-name");
}
