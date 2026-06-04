//! Integration tests for [`BearerSecret`] — exercises all variants and the
//! constant-time equality helper.

use swe_edge_ingress_grpc_auth_bearer::BearerSecret;

/// @covers: BearerSecret::Hs256 — construction
#[test]
fn test_hs256_variant_stores_raw_bytes() {
    let secret = BearerSecret::Hs256 {
        secret: b"raw-bytes".to_vec(),
    };
    if let BearerSecret::Hs256 { secret: bytes } = &secret {
        assert_eq!(bytes, b"raw-bytes");
    } else {
        panic!("expected Hs256 variant");
    }
}

/// @covers: BearerSecret::Rs256 — construction
#[test]
fn test_rs256_variant_stores_pem_bytes() {
    let pem = b"-----BEGIN PUBLIC KEY-----".to_vec();
    let secret = BearerSecret::Rs256 {
        public_pem: pem.clone(),
    };
    if let BearerSecret::Rs256 { public_pem } = &secret {
        assert_eq!(public_pem, &pem);
    } else {
        panic!("expected Rs256 variant");
    }
}

/// @covers: ct_eq_hs256 — identical secrets are equal
#[test]
fn test_ct_eq_hs256_identical_bytes_returns_true() {
    let a = BearerSecret::Hs256 {
        secret: b"abc".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"abc".to_vec(),
    };
    assert!(a.ct_eq_hs256(&b), "identical HS256 secrets must be equal");
}

/// @covers: ct_eq_hs256 — different secrets are not equal
#[test]
fn test_ct_eq_hs256_different_bytes_returns_false() {
    let a = BearerSecret::Hs256 {
        secret: b"abc".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"xyz".to_vec(),
    };
    assert!(
        !a.ct_eq_hs256(&b),
        "different HS256 secrets must not be equal"
    );
}

/// @covers: ct_eq_hs256 — variant mismatch returns false
#[test]
fn test_ct_eq_hs256_rs256_variant_returns_false() {
    let hs = BearerSecret::Hs256 {
        secret: b"key".to_vec(),
    };
    let rs = BearerSecret::Rs256 { public_pem: vec![] };
    assert!(
        !hs.ct_eq_hs256(&rs),
        "cross-variant comparison must return false"
    );
}

/// @covers: BearerSecret — Clone
#[test]
fn test_bearer_secret_clone_produces_equal_value() {
    let original = BearerSecret::Hs256 {
        secret: b"clone-me".to_vec(),
    };
    let cloned = original.clone();
    assert!(original.ct_eq_hs256(&cloned));
}

/// @covers: BearerSecret — Debug does not panic
#[test]
fn test_bearer_secret_debug_does_not_panic() {
    let s = BearerSecret::Hs256 {
        secret: b"test".to_vec(),
    };
    let dbg = format!("{s:?}");
    assert!(!dbg.is_empty());
}
