//! `VerifierProcessor` — Processor impl for the verifier crate.

/// Primary processor type for the HTTP verifier.
pub(crate) struct VerifierProcessor;

impl crate::api::traits::Processor for VerifierProcessor {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-verifier";
        LABEL
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::Processor;

    #[test]
    fn test_verifier_processor_describe_returns_label() {
        let p = VerifierProcessor;
        assert_eq!(p.describe(), "http-verifier");
    }
}
