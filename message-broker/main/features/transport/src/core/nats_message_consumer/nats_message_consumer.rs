//! NATS-backed `MessageConsumer`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::{BrokerError, MessageBroker, MessageStream};

use crate::api::port::consumer::consumer_error::ConsumerError;
use crate::api::port::consumer::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;
use crate::api::traits::Validator;

/// Consumer backed by a NATS server via `async-nats`.
///
/// Construct via [`crate::saf::nats_consumer`].
#[derive(Clone)]
pub(crate) struct NatsMessageConsumer {
    inner: Arc<dyn MessageBroker>,
}

impl NatsMessageConsumer {
    pub(crate) fn new(broker: impl MessageBroker + 'static) -> Self {
        Self {
            inner: Arc::new(broker),
        }
    }
}

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

impl Validator for NatsMessageConsumer {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
