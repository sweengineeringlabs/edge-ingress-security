//! Integration tests — MessageConsumerConfig type.

use swe_edge_ingress_message_broker_transport::MessageConsumerConfig;

/// @covers: MessageConsumerConfig — default capacity
#[test]
fn test_message_consumer_config_default_has_nonzero_capacity() {
    assert!(MessageConsumerConfig::default().capacity > 0);
}
