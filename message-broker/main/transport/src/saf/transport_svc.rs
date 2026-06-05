//! SAF factory methods on [`MessageBrokerSvc`].

use crate::api::port::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;
use crate::api::traits::validator::Validator;
use crate::api::types::message::message_broker_svc::MessageBrokerSvc;
use swe_edge_message_broker::MessageStream;

#[cfg(any(feature = "in-memory", feature = "nats"))]
use crate::api::types::message::message_consumer_handle::MessageConsumerHandle;

impl MessageBrokerSvc {
    /// Return a [`ConfigBuilderImpl`](swe_edge_configbuilder::ConfigBuilderImpl) pre-seeded
    /// with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Validate any type that implements [`Validator`].
    ///
    /// # Errors
    /// Returns the validation error string on failure.
    pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }

    /// Subscribe to `topic` using any [`MessageConsumer`].
    pub fn subscribe_to<'a>(
        consumer: &'a dyn MessageConsumer,
        topic: &'a str,
    ) -> futures::future::BoxFuture<'a, ConsumerResult<MessageStream>> {
        consumer.subscribe(topic)
    }

    /// Run a health check on any [`MessageConsumer`].
    pub fn check_health(
        consumer: &dyn MessageConsumer,
    ) -> futures::future::BoxFuture<'_, ConsumerResult<()>> {
        consumer.health_check()
    }

    /// Construct an in-memory consumer backed by a tokio broadcast channel.
    ///
    /// Returns a [`MessageConsumerHandle`] that implements [`MessageConsumer`] and
    /// can be cloned to share the same underlying channel.
    ///
    /// Requires the `in-memory` feature.
    #[cfg(feature = "in-memory")]
    pub fn default_consumer() -> MessageConsumerHandle {
        MessageConsumerHandle::new(crate::core::DefaultMessageConsumer::new())
    }

    /// Connect to a NATS server and return a consumer handle.
    ///
    /// # Errors
    /// Returns [`ConsumerError::Connection`](crate::api::error::ConsumerError::Connection)
    /// when the server is unreachable.
    ///
    /// Requires the `nats` feature.
    #[cfg(feature = "nats")]
    pub async fn nats_consumer(
        url: &str,
    ) -> Result<MessageConsumerHandle, crate::api::error::ConsumerError> {
        use swe_edge_runtime_message_broker::MessageBrokerFactory;
        let broker = MessageBrokerFactory::nats(url)
            .await
            .map_err(crate::api::error::ConsumerError::from)?;
        Ok(MessageConsumerHandle::new(
            crate::core::NatsMessageConsumer::new(broker),
        ))
    }
}
