//! `MessageConsumerHandle` — opaque handle to a live consumer instance.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::MessageStream;

use crate::api::traits::message_consumer::MessageConsumer;
use crate::api::types::consumer_result::ConsumerResult;

/// An opaque, cloneable handle to a [`MessageConsumer`] instance.
///
/// Returned by SAF factory functions such as
/// [`MessageConsumerSvc::consumer`](crate::api::types::message_broker_svc::MessageConsumerSvc).
/// Callers use this handle as a `dyn`-compatible consumer without needing to
/// name the underlying concrete type.
#[derive(Clone)]
pub struct MessageConsumerHandle {
    inner: Arc<dyn MessageConsumer>,
}

impl MessageConsumerHandle {
    /// Wrap any `MessageConsumer` implementation in a handle.
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
