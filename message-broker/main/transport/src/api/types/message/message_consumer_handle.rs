//! `MessageConsumerHandle` — opaque handle to a live consumer instance.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::MessageStream;

use crate::api::port::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;

/// An opaque, cloneable handle to a [`MessageConsumer`] instance.
///
/// Returned by SAF factory functions such as
/// [`MessageBrokerSvc::default_consumer`](crate::api::types::message::message_broker_svc::MessageBrokerSvc).
/// Callers use this handle as a `dyn`-compatible consumer without needing to
/// name the underlying concrete type.
#[derive(Clone)]
pub struct MessageConsumerHandle {
    inner: Arc<dyn MessageConsumer>,
}

impl MessageConsumerHandle {
    /// Wrap any `MessageConsumer` implementation in a handle.
    #[cfg(any(feature = "in-memory", feature = "nats"))]
    pub(crate) fn new(consumer: impl MessageConsumer + 'static) -> Self {
        Self {
            inner: Arc::new(consumer),
        }
    }
}

impl MessageConsumer for MessageConsumerHandle {
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>> {
        self.inner.subscribe(topic)
    }

    fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>> {
        self.inner.health_check()
    }
}
