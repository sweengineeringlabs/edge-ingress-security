//! Integration tests for PeerIdentity and is_reserved_key.

use std::collections::HashMap;
use swe_edge_ingress_grpc_transport::PeerIdentity;

/// @covers: PeerIdentity::is_reserved_key
#[test]
fn test_is_reserved_key_matches_x_edge_peer_prefix() {
    assert!(PeerIdentity::is_reserved_key("x-edge-peer-cn"));
    assert!(PeerIdentity::is_reserved_key("x-edge-peer-identity"));
}

/// @covers: PeerIdentity::is_reserved_key
#[test]
fn test_is_reserved_key_is_case_insensitive() {
    assert!(PeerIdentity::is_reserved_key("X-Edge-Peer-Cn"));
}

/// @covers: PeerIdentity::is_reserved_key
#[test]
fn test_is_reserved_key_does_not_match_unrelated_keys() {
    assert!(!PeerIdentity::is_reserved_key("authorization"));
    assert!(!PeerIdentity::is_reserved_key("x-edge-trace"));
}

/// @covers: PeerIdentity::empty
#[test]
fn test_peer_identity_empty_has_no_cn_or_san_or_custom_oids() {
    let id = PeerIdentity::empty();
    assert!(id.cn.is_none());
    assert!(id.san.is_empty());
    assert!(id.custom_oids.is_empty());
    assert!(id.is_empty());
}

/// @covers: PeerIdentity::is_empty
#[test]
fn test_is_empty_returns_true_for_default_identity() {
    assert!(PeerIdentity::default().is_empty());
}

/// @covers: PeerIdentity::is_empty
#[test]
fn test_peer_identity_is_not_empty_when_cn_is_populated() {
    let id = PeerIdentity {
        cn: Some("svc".into()),
        san: Vec::new(),
        custom_oids: HashMap::new(),
    };
    assert!(!id.is_empty());
}
