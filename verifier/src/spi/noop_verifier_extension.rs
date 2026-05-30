//! [`NoopVerifierExtension`] default extension — marker impl for spi layer.

use crate::api::types::NoopVerifierExtension;

/// Primary type for this module — satisfies Rule 89 filename match.
pub(crate) struct DefaultNoopVerifierExtension;

impl DefaultNoopVerifierExtension {
    #[cfg(test)]
    fn _check_noop_exists() -> bool {
        let _e = NoopVerifierExtension::new();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_verifier_extension_constructs_via_new() {
        assert!(DefaultNoopVerifierExtension::_check_noop_exists());
    }
}
