//! `DefaultValidator` — permissive no-op implementation of the `Validator` contract.

use crate::api::validator::Validator;

pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, _raw: &[u8]) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: validate
    #[test]
    fn test_validate_accepts_empty_payload() {
        assert!(DefaultValidator.validate(&[]).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_accepts_non_empty_payload() {
        assert!(DefaultValidator.validate(&[0x01, 0x02, 0x03]).is_ok());
    }
}
