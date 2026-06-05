//! `MessageConsumer` — ingress port for inbound message consumption.

use futures::future::BoxFuture;
use swe_edge_message_broker::MessageStream;

use crate::api::port::consumer_result::ConsumerResult;

/// Subscribes to topics on an external message broker and receives messages.
///
/// This is the **ingress** side of the pub/sub contract. Use the SAF factories
/// to obtain a concrete implementation:
///
/// ```rust,ignore
/// let consumer = MessageBrokerSvc::default_consumer();
/// let mut stream = consumer.subscribe("orders.created").await?;
/// while let Some(msg) = stream.next().await {
///     // handle msg
/// }
/// ```
///
/// # Feature flags
///
/// | Feature    | Backend                                   |
/// |------------|-------------------------------------------|
/// | `in-memory`| `InMemoryMessageBroker` (tokio broadcast) |
/// | `nats`     | `NatsMessageBroker` (async-nats)          |
pub trait MessageConsumer: Send + Sync {
    /// Subscribe to `topic` and return a lazy stream of incoming messages.
    ///
    /// The stream yields `Ok(Message)` for each received message and
    /// `Err(ConsumerError::StreamLagged)` when the internal buffer overflows.
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>>;

    /// Verify the consumer is connected and the broker is reachable.
    fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>>;
}
