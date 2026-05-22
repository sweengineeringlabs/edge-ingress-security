//! Integration tests for [`MtlsAuthConfig`].

use swe_edge_ingress_grpc_auth_mtls::*;

/// @covers: allow_any_verified_peer
#[test]
fn test_default_accepts_any_verified_peer_with_no_bypass() {
    let cfg = MtlsAuthConfig::default();
    assert!(cfg.allowed_cns.is_empty());
    assert!(cfg.allowed_san_dns.is_empty());
    assert!(!cfg.allow_unauthenticated_methods);
    assert!(cfg.unauthenticated_methods.is_empty());
}

/// @covers: allow_any_verified_peer
#[test]
fn test_allow_any_verified_peer_returns_default_config() {
    let cfg = MtlsAuthConfig::allow_any_verified_peer();
    assert!(cfg.allowed_cns.is_empty());
    assert!(cfg.allowed_san_dns.is_empty());
    assert!(!cfg.allow_unauthenticated_methods);
}

/// @covers: restrict_to_cns
#[test]
fn test_restrict_to_cns_populates_only_cn_allowlist() {
    let cfg = MtlsAuthConfig::restrict_to_cns(["alice".to_string(), "bob".to_string()]);
    assert_eq!(
        cfg.allowed_cns,
        vec!["alice".to_string(), "bob".to_string()]
    );
    assert!(cfg.allowed_san_dns.is_empty());
}

/// @covers: from_toml
#[test]
fn test_from_toml_round_trips_documented_keys() {
    let toml_src = r#"
        allowed_cns           = ["alice", "bob"]
        allowed_san_dns       = ["svc-a.local"]
        allow_unauthenticated_methods = true
        unauthenticated_methods       = ["/grpc.health.v1.Health/Check"]
    "#;
    let cfg = MtlsAuthConfig::from_toml(toml_src).expect("toml parses");
    assert_eq!(cfg.allowed_cns.len(), 2);
    assert_eq!(cfg.allowed_san_dns.len(), 1);
    assert!(cfg.allow_unauthenticated_methods);
    assert_eq!(cfg.unauthenticated_methods.len(), 1);
}

/// @covers: from_toml
#[test]
fn test_from_toml_missing_keys_default_to_empty_lists() {
    let cfg = MtlsAuthConfig::from_toml("").expect("empty toml parses");
    assert!(cfg.allowed_cns.is_empty());
}
