//! In-memory `MessageConsumer` backed by the runtime's in-memory broker backend.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::{MessageBroker, MessageStream};
use swe_edge_runtime_message_broker::MessageBrokerFactory;

use crate::api::error::consumer_error::ConsumerError;
use crate::api::port::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;

/// In-process consumer backed by a tokio broadcast channel.
///
/// Suitable for single-process deployments, integration tests, and development.
#[derive(Clone)]
pub(crate) struct DefaultMessageConsumer {
    inner: Arc<dyn MessageBroker>,
}

impl DefaultMessageConsumer {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(MessageBrokerFactory::in_memory()),
        }
    }
}

impl crate::api::default::consumer::in_memory_message_consumer::InMemoryMessageConsumer
    for DefaultMessageConsumer
{
}

// Name the api/ marker (SEA rule 121) in a type position so it stays a live
// part of the contract; the empty impl above proves the concrete consumer
// conforms to it.
const _: core::marker::PhantomData<
    dyn crate::api::default::consumer::in_memory_message_consumer::InMemoryMessageConsumer,
> = core::marker::PhantomData;

impl MessageConsumer for DefaultMessageConsumer {
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>> {
        Box::pin(async move {
            self.inner
                .subscribe(topic)
                .await
                .map_err(ConsumerError::from)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>> {
        Box::pin(async move { self.inner.health_check().await.map_err(ConsumerError::from) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message_consumer_new_is_constructible() {
        let _ = DefaultMessageConsumer::new();
    }

    #[tokio::test]
    async fn test_default_message_consumer_health_check_returns_ok() {
        let c = DefaultMessageConsumer::new();
        assert!(c.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_default_message_consumer_subscribe_to_topic_succeeds() {
        let c = DefaultMessageConsumer::new();
        assert!(c.subscribe("test.topic").await.is_ok());
    }
}
