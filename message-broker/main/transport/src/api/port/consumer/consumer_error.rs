//! Error type for inbound message consumption operations.

use swe_edge_message_broker::BrokerError;

/// Errors returned by [`MessageConsumer`](super::super::message_consumer::MessageConsumer) operations.
#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    /// Failed to subscribe to the given topic.
    #[error("subscribe failed on topic '{topic}': {reason}")]
    Subscribe { topic: String, reason: String },
    /// Consumer is not connected or the broker is unreachable.
    #[error("consumer unavailable: {0}")]
    Unavailable(String),
    /// Connection to the broker failed.
    #[error("connection failed: {0}")]
    Connection(String),
    /// The message stream lagged and messages were dropped.
    #[error("stream lagged: {0} messages dropped")]
    StreamLagged(u64),
}

impl From<BrokerError> for ConsumerError {
    fn from(e: BrokerError) -> Self {
        match e {
            BrokerError::Subscribe { topic, reason } => ConsumerError::Subscribe { topic, reason },
            BrokerError::Unavailable(m) => ConsumerError::Unavailable(m),
            BrokerError::Connection(m) => ConsumerError::Connection(m),
            BrokerError::StreamLagged { count } => ConsumerError::StreamLagged(count),
            other => ConsumerError::Unavailable(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_error_subscribe_formats_topic_and_reason() {
        let e = ConsumerError::Subscribe {
            topic: "t".into(),
            reason: "r".into(),
        };
        assert!(e.to_string().contains("t"));
        assert!(e.to_string().contains("r"));
    }

    #[test]
    fn test_consumer_error_from_broker_error_subscribe() {
        let be = BrokerError::Subscribe {
            topic: "events".into(),
            reason: "down".into(),
        };
        assert!(matches!(
            ConsumerError::from(be),
            ConsumerError::Subscribe { .. }
        ));
    }

    #[test]
    fn test_consumer_error_from_broker_error_stream_lagged() {
        let be = BrokerError::StreamLagged { count: 5 };
        assert!(matches!(
            ConsumerError::from(be),
            ConsumerError::StreamLagged(5)
        ));
    }
}
