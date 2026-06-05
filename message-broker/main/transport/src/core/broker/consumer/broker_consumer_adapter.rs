//! Broker-adapter `MessageConsumer`.
//!
//! Wraps any injected [`MessageBroker`] as a [`MessageConsumer`]. The assembler
//! injects the backend; this crate never constructs one itself.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::{MessageBroker, MessageStream};

use crate::api::error::consumer_error::ConsumerError;
use crate::api::traits::message_consumer::MessageConsumer;
use crate::api::types::consumer_result::ConsumerResult;

/// Adapts any injected [`MessageBroker`] to the [`MessageConsumer`] port contract.
///
/// Construct via [`crate::saf::MessageConsumerSvc::from_broker`].
#[derive(Clone)]
pub(crate) struct BrokerConsumerAdapter {
    inner: Arc<dyn MessageBroker>,
}

impl BrokerConsumerAdapter {
    /// Wrap an already-constructed [`MessageBroker`] as a consumer.
    pub(crate) fn new(broker: impl MessageBroker + 'static) -> Self {
        Self {
            inner: Arc::new(broker),
        }
    }
}

impl crate::api::broker::consumer::broker_message_consumer::BrokerMessageConsumer
    for BrokerConsumerAdapter
{
}

// Name the api/ marker (SEA rule 121) in a type position so it stays a live
// part of the contract; the empty impl above proves the concrete consumer
// conforms to it.
const _: core::marker::PhantomData<
    dyn crate::api::broker::consumer::broker_message_consumer::BrokerMessageConsumer,
> = core::marker::PhantomData;

impl MessageConsumer for BrokerConsumerAdapter {
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

    use swe_edge_message_broker::BrokerError;

    struct MockBroker;
    impl MessageBroker for MockBroker {
        fn publish<'a>(
            &'a self,
            _: &'a str,
            _: swe_edge_message_broker::Message,
        ) -> futures::future::BoxFuture<'a, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
        fn subscribe<'a>(
            &'a self,
            _: &'a str,
        ) -> futures::future::BoxFuture<'a, Result<MessageStream, BrokerError>> {
            Box::pin(futures::future::ready(Ok(
                Box::pin(futures::stream::empty()) as MessageStream,
            )))
        }
        fn health_check(&self) -> futures::future::BoxFuture<'_, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    #[test]
    fn test_broker_consumer_adapter_is_object_safe() {
        fn _assert(_: &dyn MessageConsumer) {}
    }

    #[test]
    fn test_broker_consumer_adapter_new_accepts_any_broker() {
        let _ = BrokerConsumerAdapter::new(MockBroker);
    }

    #[tokio::test]
    async fn test_broker_consumer_adapter_health_check_returns_ok() {
        let c = BrokerConsumerAdapter::new(MockBroker);
        assert!(c.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_broker_consumer_adapter_subscribe_to_topic_returns_ok() {
        let c = BrokerConsumerAdapter::new(MockBroker);
        assert!(c.subscribe("test.topic").await.is_ok());
    }
}
