//! Integration tests for PeerIdentity and is_reserved_peer_key.

use std::collections::HashMap;
use swe_edge_ingress_grpc_transport::{is_reserved_peer_key, PeerIdentity};

/// @covers: is_reserved_peer_key
#[test]
fn test_is_reserved_peer_key_matches_x_edge_peer_prefix() {
    assert!(is_reserved_peer_key("x-edge-peer-cn"));
    assert!(is_reserved_peer_key("x-edge-peer-identity"));
}

/// @covers: is_reserved_peer_key
#[test]
fn test_is_reserved_peer_key_is_case_insensitive() {
    assert!(is_reserved_peer_key("X-Edge-Peer-Cn"));
}

/// @covers: is_reserved_peer_key
#[test]
fn test_is_reserved_peer_key_does_not_match_unrelated_keys() {
    assert!(!is_reserved_peer_key("authorization"));
    assert!(!is_reserved_peer_key("x-edge-trace"));
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
