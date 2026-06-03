//! Integration tests for `ApiKeyVerifier`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_verifier::{ApiKeyVerifier, VerifierError};

fn verifier() -> ApiKeyVerifier {
    ApiKeyVerifier::new(vec![b"prod-key-alpha".to_vec(), b"prod-key-beta".to_vec()])
}

/// @covers: ApiKeyVerifier — registered key is accepted.
#[test]
fn test_api_key_verifier_registered_key_accepted() {
    assert!(verifier().verify(b"prod-key-alpha").is_ok());
    assert!(verifier().verify(b"prod-key-beta").is_ok());
}

/// @covers: ApiKeyVerifier — unregistered key returns UnknownApiKey.
#[test]
fn test_api_key_verifier_unregistered_key_rejected() {
    let err = verifier().verify(b"not-a-key").unwrap_err();
    assert!(matches!(err, VerifierError::UnknownApiKey));
}

/// @covers: ApiKeyVerifier — prefix of valid key is rejected (length mismatch).
#[test]
fn test_api_key_verifier_prefix_rejected() {
    assert!(verifier().verify(b"prod-key").is_err());
}

/// @covers: ApiKeyVerifier — empty verifier rejects all inputs.
#[test]
fn test_api_key_verifier_empty_set_rejects_all() {
    let v = ApiKeyVerifier::new(vec![]);
    assert!(v.is_empty());
    assert!(matches!(
        v.verify(b"anything"),
        Err(VerifierError::UnknownApiKey)
    ));
}

/// @covers: ApiKeyVerifier — len / is_empty reflect key count.
#[test]
fn test_api_key_verifier_len_and_is_empty() {
    let v = verifier();
    assert_eq!(v.len(), 2);
    assert!(!v.is_empty());
}
