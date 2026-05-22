//! Integration tests for [`ApplicationConfigBuilder`].

use swe_edge_ingress_grpc_auth_mtls::ApplicationConfigBuilder;

/// @covers: new
#[test]
fn test_new_creates_builder_with_empty_config() {
    let cfg = ApplicationConfigBuilder::new().build();
    assert!(cfg.allowed_cns.is_empty());
    assert!(!cfg.allow_unauthenticated_methods);
}

/// @covers: allowed_cns
#[test]
fn test_allowed_cns_sets_cn_allowlist() {
    let cfg = ApplicationConfigBuilder::new()
        .allowed_cns(vec!["svc-a".to_string()])
        .build();
    assert_eq!(cfg.allowed_cns, vec!["svc-a".to_string()]);
}

/// @covers: require_peer_cert
#[test]
fn test_require_peer_cert_false_enables_unauthenticated_methods() {
    let cfg = ApplicationConfigBuilder::new()
        .require_peer_cert(false)
        .build();
    assert!(cfg.allow_unauthenticated_methods);
}
