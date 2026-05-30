//! `ApiKeyVerifier` — constant-time inbound API key validation.

use subtle::ConstantTimeEq;

use crate::api::error::VerifierError;

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
        Self {
            valid_keys: valid_keys.into_iter().collect(),
        }
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
    pub fn len(&self) -> usize {
        self.valid_keys.len()
    }

    /// Whether the verifier has no registered keys.
    pub fn is_empty(&self) -> bool {
        self.valid_keys.is_empty()
    }
}
