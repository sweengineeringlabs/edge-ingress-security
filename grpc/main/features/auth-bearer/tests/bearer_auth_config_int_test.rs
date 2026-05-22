//! Integration tests for [`BearerSecret`] and [`BearerIngressConfig`].

use swe_edge_ingress_grpc_auth_bearer::*;

/// @covers: ct_eq_hs256
#[test]
fn test_ct_eq_hs256_returns_true_for_identical_secrets() {
    let a = BearerSecret::Hs256 {
        secret: b"secret".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"secret".to_vec(),
    };
    assert!(a.ct_eq_hs256(&b));
}

/// @covers: ct_eq_hs256
#[test]
fn test_ct_eq_hs256_returns_false_for_different_secrets() {
    let a = BearerSecret::Hs256 {
        secret: b"alpha".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"beta".to_vec(),
    };
    assert!(!a.ct_eq_hs256(&b));
}

/// @covers: ct_eq_hs256
#[test]
fn test_ct_eq_hs256_returns_false_for_variant_mismatch() {
    let a = BearerSecret::Hs256 {
        secret: b"x".to_vec(),
    };
    let b = BearerSecret::Rs256 { public_pem: vec![] };
    assert!(!a.ct_eq_hs256(&b));
}
