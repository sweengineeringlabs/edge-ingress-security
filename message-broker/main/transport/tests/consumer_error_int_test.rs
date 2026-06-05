//! Integration tests — ConsumerError type.

use swe_edge_ingress_message_broker_transport::ConsumerError;

/// @covers: ConsumerError::Subscribe
#[test]
fn test_consumer_error_subscribe_formats_topic_and_reason() {
    let e = ConsumerError::Subscribe {
        topic: "orders".into(),
        reason: "broker down".into(),
    };
    let s = e.to_string();
    assert!(s.contains("orders"), "should contain topic");
    assert!(s.contains("broker down"), "should contain reason");
}

/// @covers: ConsumerError::Unavailable
#[test]
fn test_consumer_error_unavailable_formats_message() {
    let e = ConsumerError::Unavailable("offline".into());
    assert!(e.to_string().contains("offline"));
}

/// @covers: ConsumerError::Connection
#[test]
fn test_consumer_error_connection_formats_message() {
    let e = ConsumerError::Connection("refused".into());
    assert!(e.to_string().contains("refused"));
}

/// @covers: ConsumerError::StreamLagged
#[test]
fn test_consumer_error_stream_lagged_formats_count() {
    let e = ConsumerError::StreamLagged(7);
    assert!(e.to_string().contains("7"));
}

/// @covers: ConsumerError::from(BrokerError)
#[test]
fn test_consumer_error_from_broker_error_subscribe() {
    use swe_edge_message_broker::BrokerError;
    let be = BrokerError::Subscribe {
        topic: "events".into(),
        reason: "nats down".into(),
    };
    let ce = ConsumerError::from(be);
    assert!(matches!(ce, ConsumerError::Subscribe { .. }));
}

/// @covers: ConsumerError::from(BrokerError)
#[test]
fn test_consumer_error_from_broker_error_stream_lagged() {
    use swe_edge_message_broker::BrokerError;
    let be = BrokerError::StreamLagged { count: 5 };
    let ce = ConsumerError::from(be);
    assert!(matches!(ce, ConsumerError::StreamLagged(5)));
}
