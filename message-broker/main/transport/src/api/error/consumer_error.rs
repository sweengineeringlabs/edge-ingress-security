//! Error type for inbound message consumption operations.

use swe_edge_message_broker::BrokerError;

/// Errors returned by [`MessageConsumer`](crate::api::port::MessageConsumer) operations.
#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    /// Failed to subscribe to the given topic.
    #[error("subscribe failed on topic '{topic}': {reason}")]
    Subscribe {
        /// The topic that the subscribe was attempted on.
        topic: String,
        /// The reason for the failure.
        reason: String,
    },
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
