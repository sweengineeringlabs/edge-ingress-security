//! Integration tests — Validator trait via SAF.

use swe_edge_ingress_message_broker_transport::{validate, MessageConsumerConfig, Validator};

/// @covers: validate — delegates to the Validator impl; non-zero default capacity passes.
#[test]
fn test_validate_default_consumer_config_capacity_nonzero_returns_ok() {
    struct CfgValidator(MessageConsumerConfig);
    impl Validator for CfgValidator {
        fn validate(&self) -> Result<(), String> {
            if self.0.capacity == 0 {
                Err("capacity must be > 0".into())
            } else {
                Ok(())
            }
        }
    }

    assert!(validate(&CfgValidator(MessageConsumerConfig::default())).is_ok());
}

/// @covers: validate — propagates Err from the Validator impl.
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
