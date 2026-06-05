//! Integration tests — MessageConsumerSvc factory type.

use swe_edge_ingress_message_consumer::MessageConsumerSvc;

/// @covers: MessageConsumerSvc — type exists and is constructible
#[test]
fn test_message_consumer_svc_type_exists() {
    // MessageConsumerSvc is a unit struct; it exists and the type is accessible.
    let _: MessageConsumerSvc = MessageConsumerSvc;
}
