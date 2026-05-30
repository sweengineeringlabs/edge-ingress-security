//! `NoopVerifierExtension` — default no-op extension placeholder.

/// Default no-op extension for downstream verifier customisation.
pub struct NoopVerifierExtension;

impl NoopVerifierExtension {
    /// Construct a new no-op extension.
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopVerifierExtension {
    fn default() -> Self {
        Self::new()
    }
}
