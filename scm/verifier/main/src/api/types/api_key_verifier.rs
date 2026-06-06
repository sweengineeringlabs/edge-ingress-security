//! `ApiKeyVerifier` — constant-time inbound API key validation.

use subtle::ConstantTimeEq;

use crate::api::error::VerifierError;

/// Validates inbound API keys against a configured set using constant-time
/// byte comparison, preventing timing-based enumeration attacks.
///
/// Keys are stored as raw bytes; callers encode them (hex, base64, plain)
/// before constructing the verifier. Comparison time is the same regardless
/// of which key matches, preventing timing oracle attacks.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_verifier::ApiKeyVerifier;
///
/// let verifier = ApiKeyVerifier::new(vec![
///     b"secret-key-one".to_vec(),
///     b"secret-key-two".to_vec(),
/// ]);
///
/// assert_eq!(verifier.len(), 2);
/// assert!(!verifier.is_empty());
/// assert!(verifier.verify(b"secret-key-one").is_ok());
/// assert!(verifier.verify(b"unknown-key").is_err());
/// ```
#[derive(Clone)]
pub struct ApiKeyVerifier {
    valid_keys: Vec<Vec<u8>>,
}

impl ApiKeyVerifier {
    /// Construct from a list of valid key bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_verifier::ApiKeyVerifier;
    /// let v = ApiKeyVerifier::new(vec![b"my-key".to_vec()]);
    /// assert_eq!(v.len(), 1);
    /// ```
    pub fn new(valid_keys: impl IntoIterator<Item = Vec<u8>>) -> Self {
        Self {
            valid_keys: valid_keys.into_iter().collect(),
        }
    }

    /// Verify `key` against the configured set.
    ///
    /// Returns `Ok(())` on the first constant-time match, or
    /// [`VerifierError::UnknownApiKey`] if no key matches.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_verifier::ApiKeyVerifier;
    /// let v = ApiKeyVerifier::new(vec![b"valid".to_vec()]);
    /// assert!(v.verify(b"valid").is_ok());
    /// assert!(v.verify(b"invalid").is_err());
    /// ```
    pub fn verify(&self, key: &[u8]) -> Result<(), VerifierError> {
        for valid in &self.valid_keys {
            if key.ct_eq(valid.as_slice()).into() {
                return Ok(());
            }
        }
        Err(VerifierError::UnknownApiKey)
    }

    /// Number of registered keys.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_verifier::ApiKeyVerifier;
    /// assert_eq!(ApiKeyVerifier::new(vec![b"a".to_vec(), b"b".to_vec()]).len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.valid_keys.len()
    }

    /// Whether the verifier has no registered keys.
    ///
    /// An empty verifier rejects every key — it is safe to construct but
    /// not useful in production.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_verifier::ApiKeyVerifier;
    /// assert!(ApiKeyVerifier::new(vec![]).is_empty());
    /// assert!(!ApiKeyVerifier::new(vec![b"k".to_vec()]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.valid_keys.is_empty()
    }
}
