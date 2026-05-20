//! Default Validator implementation.

use crate::api::traits::Validator;

/// Passthrough validator — always valid.
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_validator_always_returns_ok() {
        assert!(DefaultValidator.validate().is_ok());
    }
}
