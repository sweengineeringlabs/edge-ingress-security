//! NATS-backed `MessageConsumer`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::{MessageBroker, MessageStream};

use crate::api::error::consumer_error::ConsumerError;
use crate::api::port::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;

/// Consumer backed by a NATS server via `async-nats`.
///
/// Construct via [`crate::saf::MessageBrokerSvc::nats_consumer`].
#[derive(Clone)]
pub(crate) struct NatsMessageConsumer {
    inner: Arc<dyn MessageBroker>,
}

impl NatsMessageConsumer {
    /// Wrap an already-connected [`MessageBroker`] as a consumer.
    pub(crate) fn new(broker: impl MessageBroker + 'static) -> Self {
        Self {
            inner: Arc::new(broker),
        }
    }
}

impl crate::api::nats::consumer::nats_message_consumer::NatsMessageConsumer
    for NatsMessageConsumer
{
}

// Name the api/ marker (SEA rule 121) in a type position so it stays a live
// part of the contract; the empty impl above proves the concrete consumer
// conforms to it.
const _: core::marker::PhantomData<
    dyn crate::api::nats::consumer::nats_message_consumer::NatsMessageConsumer,
> = core::marker::PhantomData;

impl MessageConsumer for NatsMessageConsumer {
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

    struct NatsMessageConsumerMockBroker;
    impl MessageBroker for NatsMessageConsumerMockBroker {
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
    fn test_nats_message_consumer_is_object_safe() {
        fn _assert(_: &dyn MessageConsumer) {}
    }

    #[test]
    fn test_nats_message_consumer_new_accepts_any_broker() {
        let _ = NatsMessageConsumer::new(NatsMessageConsumerMockBroker);
    }
}
