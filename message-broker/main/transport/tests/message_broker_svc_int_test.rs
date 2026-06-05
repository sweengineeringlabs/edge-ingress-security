//! Integration tests — MessageBrokerSvc factory type.

use swe_edge_ingress_message_broker_transport::MessageBrokerSvc;

/// @covers: MessageBrokerSvc — type exists and is constructible
#[test]
fn test_message_broker_svc_type_exists() {
    // MessageBrokerSvc is a unit struct; it exists and the type is accessible.
    let _: MessageBrokerSvc = MessageBrokerSvc;
}
