//! Integration tests — Validator trait via SAF.

use swe_edge_ingress_message_broker_transport::{
    MessageBrokerSvc, MessageConsumerConfig, Validator,
};

/// @covers: MessageBrokerSvc::validate — delegates to the Validator impl; non-zero default capacity passes.
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

    assert!(MessageBrokerSvc::validate(&CfgValidator(MessageConsumerConfig::default())).is_ok());
}

/// @covers: MessageBrokerSvc::validate — propagates Err from the Validator impl.
#[test]
fn test_validate_returns_err_for_zero_capacity() {
    struct ZeroCapacity;
    impl Validator for ZeroCapacity {
        fn validate(&self) -> Result<(), String> {
            Err("capacity must be > 0".into())
        }
    }
    assert!(MessageBrokerSvc::validate(&ZeroCapacity).is_err());
}
