//! `ApiKeyVerifier` — constant-time inbound API key validation.

use subtle::ConstantTimeEq;

use crate::api::verifier_error::VerifierError;

/// Validates inbound API keys against a configured set using constant-time
/// byte comparison, preventing timing-based enumeration attacks.
///
/// Keys are stored as raw bytes; callers encode them (hex, base64, plain)
/// before constructing the verifier.
#[derive(Clone)]
pub struct ApiKeyVerifier {
    valid_keys: Vec<Vec<u8>>,
}

impl ApiKeyVerifier {
    /// Construct from a list of valid key bytes.
    pub fn new(valid_keys: impl IntoIterator<Item = Vec<u8>>) -> Self {
        Self { valid_keys: valid_keys.into_iter().collect() }
    }

    /// Verify `key` against the configured set.
    ///
    /// Returns `Ok(())` on the first constant-time match, or
    /// [`VerifierError::UnknownApiKey`] if no key matches.
    pub fn verify(&self, key: &[u8]) -> Result<(), VerifierError> {
        for valid in &self.valid_keys {
            if key.ct_eq(valid.as_slice()).into() {
                return Ok(());
            }
        }
        Err(VerifierError::UnknownApiKey)
    }

    /// Number of registered keys.
    pub fn len(&self) -> usize { self.valid_keys.len() }

    /// Whether the verifier has no registered keys.
    pub fn is_empty(&self) -> bool { self.valid_keys.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verifier() -> ApiKeyVerifier {
        ApiKeyVerifier::new(vec![b"key-alpha".to_vec(), b"key-beta".to_vec()])
    }

    /// @covers: verify — matching key returns Ok.
    #[test]
    fn test_verify_returns_ok_for_matching_key() {
        assert!(verifier().verify(b"key-alpha").is_ok());
        assert!(verifier().verify(b"key-beta").is_ok());
    }

    /// @covers: verify — unknown key returns UnknownApiKey.
    #[test]
    fn test_verify_returns_unknown_api_key_for_invalid_key() {
        let err = verifier().verify(b"bad-key").unwrap_err();
        assert!(matches!(err, VerifierError::UnknownApiKey));
    }

    /// @covers: verify — prefix of valid key is rejected.
    #[test]
    fn test_verify_rejects_prefix_of_valid_key() {
        assert!(verifier().verify(b"key").is_err());
    }

    /// @covers: new — empty verifier rejects all keys.
    #[test]
    fn test_new_empty_verifier_rejects_all_keys() {
        let v = ApiKeyVerifier::new(vec![]);
        assert!(v.is_empty());
        assert!(v.verify(b"anything").is_err());
    }

    /// @covers: len, is_empty.
    #[test]
    fn test_len_and_is_empty_reflect_key_count() {
        let v = verifier();
        assert_eq!(v.len(), 2);
        assert!(!v.is_empty());
    }
}
