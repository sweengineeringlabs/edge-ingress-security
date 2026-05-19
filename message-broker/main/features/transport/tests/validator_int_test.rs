//! Integration tests — Validator trait via SAF.

use swe_edge_ingress_message_broker::{validate, ApplicationConfigBuilder, Validator};

#[test]
fn test_validate_application_config_builder_returns_ok() {
    struct CfgValidator(ApplicationConfigBuilder);
    impl Validator for CfgValidator {
        fn validate(&self) -> Result<(), String> {
            if self.0.capacity == 0 {
                Err("capacity must be > 0".into())
            } else {
                Ok(())
            }
        }
    }
    assert!(validate(&CfgValidator(ApplicationConfigBuilder::new())).is_ok());
}

#[test]
fn test_validate_returns_err_for_zero_capacity() {
    struct Zero;
    impl Validator for Zero {
        fn validate(&self) -> Result<(), String> {
            Err("zero".into())
        }
    }
    assert!(validate(&Zero).is_err());
}
