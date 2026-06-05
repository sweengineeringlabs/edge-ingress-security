//! Integration tests — ApplicationConfigBuilder.

use swe_edge_ingress_message_broker_transport::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder::new
#[test]
fn test_application_config_builder_new_returns_default_capacity() {
    let b = ApplicationConfigBuilder::new();
    assert_eq!(b.capacity, 1024);
}

/// @covers: ApplicationConfigBuilder::with_capacity
#[test]
fn test_application_config_builder_with_capacity_overrides_default() {
    let b = ApplicationConfigBuilder::new().with_capacity(512);
    assert_eq!(b.capacity, 512);
}

/// @covers: ApplicationConfigBuilder::default
#[test]
fn test_application_config_builder_default_equals_new() {
    let default = ApplicationConfigBuilder::default();
    let explicit = ApplicationConfigBuilder::new();
    assert_eq!(default.capacity, explicit.capacity);
}
