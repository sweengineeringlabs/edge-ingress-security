//! Integration tests — MessageConsumerConfig.

use swe_edge_ingress_message_broker_transport::MessageConsumerConfig;

/// @covers: MessageConsumerConfig::default
#[test]
fn test_message_consumer_config_default_capacity_is_1024() {
    assert_eq!(MessageConsumerConfig::default().capacity, 1024);
}

/// @covers: MessageConsumerConfig — ConfigSection::section_name
#[test]
fn test_message_consumer_config_section_name_is_message_consumer() {
    use swe_edge_configbuilder::ConfigSection;
    assert_eq!(MessageConsumerConfig::section_name(), "message_consumer");
}
