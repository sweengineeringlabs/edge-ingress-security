//! SEA interface contract — inbound transport traits.

/// Validates an inbound configuration or request value.
///
/// All inbound port implementors must also implement this trait to satisfy
/// SEA rule 155 (every non-orchestrator crate must have a `Validator` in `api/traits.rs`).
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a human-readable error.
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self) -> Result<(), String> {
            Err("invalid".into())
        }
    }

    #[test]
    fn test_validator_ok_returns_unit() {
        assert!(AlwaysValid.validate().is_ok());
    }

    #[test]
    fn test_validator_err_returns_message() {
        let e = AlwaysInvalid.validate().unwrap_err();
        assert!(!e.is_empty());
    }
}
