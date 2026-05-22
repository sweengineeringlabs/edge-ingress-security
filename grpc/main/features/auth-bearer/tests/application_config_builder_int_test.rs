//! Integration tests for [`ApplicationConfigBuilder`].

use swe_edge_ingress_grpc_auth_bearer::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder::new
#[test]
fn test_new_returns_default_builder_without_panic() {
    let _builder = ApplicationConfigBuilder::new();
}

/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_build_consumes_builder_and_returns_value() {
    let cfg = ApplicationConfigBuilder::new().build();
    // Debug formatting must succeed (no panic).
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("ApplicationConfigBuilder"));
}

/// @covers: ApplicationConfigBuilder — Default impl
#[test]
fn test_default_produces_same_value_as_new() {
    let via_new = ApplicationConfigBuilder::new();
    let via_default = ApplicationConfigBuilder::default();
    // Both should produce the same Debug representation.
    assert_eq!(format!("{via_new:?}"), format!("{via_default:?}"));
}
