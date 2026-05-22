//! Integration tests for ApplicationConfig.

use swe_edge_ingress_grpc_transport::ApplicationConfig;

/// @covers: ApplicationConfig::default
#[test]
fn test_application_config_default_has_empty_fields() {
    let cfg = ApplicationConfig::default();
    assert_eq!(cfg.name, "");
    assert_eq!(cfg.version, "");
}
