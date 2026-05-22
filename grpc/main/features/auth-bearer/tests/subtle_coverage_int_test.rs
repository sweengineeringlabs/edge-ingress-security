//! Dependency-coverage test for `subtle`.
//!
//! `subtle` is used by [`BearerSecret::ct_eq_hs256`] to perform
//! constant-time comparison of HS256 secrets, preventing timing-oracle
//! attacks on secret material.  These tests exercise `subtle::ConstantTimeEq`
//! directly and also via the public `BearerSecret` API.

use subtle::ConstantTimeEq;

use swe_edge_ingress_grpc_auth_bearer::BearerSecret;

/// @covers: subtle — ConstantTimeEq on raw byte slices works correctly
#[test]
fn test_subtle_constant_time_eq_same_bytes_returns_choice_true() {
    let a: &[u8] = b"token-secret";
    let b: &[u8] = b"token-secret";
    let ct_result: subtle::Choice = a.ct_eq(b);
    assert!(
        bool::from(ct_result),
        "ConstantTimeEq on identical slices must return true"
    );
}

/// @covers: subtle — ConstantTimeEq on different bytes returns false
#[test]
fn test_subtle_constant_time_eq_different_bytes_returns_choice_false() {
    let a: &[u8] = b"alpha";
    let b: &[u8] = b"beta!";
    let ct_result: subtle::Choice = a.ct_eq(b);
    assert!(
        !bool::from(ct_result),
        "ConstantTimeEq on different slices must return false"
    );
}

/// @covers: subtle — BearerSecret::ct_eq_hs256 identical secrets compare equal
#[test]
fn test_subtle_ct_eq_hs256_identical_secrets_returns_true() {
    let a = BearerSecret::Hs256 {
        secret: b"constant-time-safe".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"constant-time-safe".to_vec(),
    };
    assert!(
        a.ct_eq_hs256(&b),
        "subtle::ConstantTimeEq must return true for identical secrets"
    );
}

/// @covers: subtle — BearerSecret::ct_eq_hs256 different secrets compare not-equal
#[test]
fn test_subtle_ct_eq_hs256_different_secrets_returns_false() {
    let a = BearerSecret::Hs256 {
        secret: b"secret-one".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"secret-two".to_vec(),
    };
    assert!(
        !a.ct_eq_hs256(&b),
        "subtle::ConstantTimeEq must return false for different secrets"
    );
}

/// @covers: subtle — cross-variant comparison always returns false
#[test]
fn test_subtle_ct_eq_hs256_cross_variant_always_returns_false() {
    let hs = BearerSecret::Hs256 {
        secret: b"key".to_vec(),
    };
    let rs = BearerSecret::Rs256 {
        public_pem: b"key".to_vec(),
    };
    assert!(
        !hs.ct_eq_hs256(&rs),
        "algorithm variant mismatch must always return false regardless of key bytes"
    );
}
