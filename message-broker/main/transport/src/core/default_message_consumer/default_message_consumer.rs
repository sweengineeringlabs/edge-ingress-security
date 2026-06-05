//! In-memory `MessageConsumer` backed by `swe_edge_runtime_message_broker::in_memory_broker()`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_runtime_message_broker::{in_memory_broker, MessageBroker, MessageStream};

use crate::api::port::consumer::consumer_error::ConsumerError;
use crate::api::port::consumer::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;
use crate::api::traits::Validator;

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
            inner: Arc::new(in_memory_broker()),
        }
    }
}

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

impl Validator for DefaultMessageConsumer {
    fn validate(&self) -> Result<(), String> {
        Ok(())
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
        assert!(DefaultMessageConsumer::new().health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_default_message_consumer_subscribe_returns_stream() {
        let c = DefaultMessageConsumer::new();
        assert!(c.subscribe("test.topic").await.is_ok());
    }
}
