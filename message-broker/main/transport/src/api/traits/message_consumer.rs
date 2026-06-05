//! `MessageConsumer` — ingress trait for inbound message consumption.

use futures::future::BoxFuture;
use swe_edge_message_broker::MessageStream;

use crate::api::types::consumer_result::ConsumerResult;

/// Subscribes to topics on an external message broker and receives messages.
///
/// Use the SAF factories to obtain a concrete implementation:
///
/// ```rust,ignore
/// let consumer = MessageConsumerSvc::from_broker(broker);
/// let mut stream = consumer.subscribe("orders.created").await?;
/// while let Some(msg) = stream.next().await {
///     // handle msg
/// }
/// ```
pub trait MessageConsumer: Send + Sync {
    /// Subscribe to `topic` and return a lazy stream of incoming messages.
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>>;

    /// Verify the consumer is connected and the broker is reachable.
    fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>>;
}
