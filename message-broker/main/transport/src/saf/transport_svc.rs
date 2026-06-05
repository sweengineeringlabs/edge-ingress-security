//! SAF factory methods on [`MessageConsumerSvc`].

use crate::api::traits::message_consumer::MessageConsumer;
use crate::api::types::consumer_result::ConsumerResult;
use crate::api::traits::validator::Validator;
use crate::api::types::message::message_broker_svc::MessageConsumerSvc;
use crate::api::types::message::message_consumer_handle::MessageConsumerHandle;
use swe_edge_message_broker::{MessageBroker, MessageStream};

impl MessageConsumerSvc {
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

    /// Wrap an already-constructed [`MessageConsumer`] in a handle for injection.
    ///
    /// Use this when the assembler has already constructed and configured the consumer.
    pub fn consumer(c: impl MessageConsumer + 'static) -> MessageConsumerHandle {
        MessageConsumerHandle::new(c)
    }

    /// Wrap an already-constructed [`MessageBroker`] as a consumer handle.
    ///
    /// The `BrokerConsumerAdapter` adapts the broker's subscribe/health_check to
    /// the [`MessageConsumer`] contract. The backend is fully owned by the caller —
    /// this crate never constructs runtime brokers itself.
    pub fn from_broker(b: impl MessageBroker + 'static) -> MessageConsumerHandle {
        MessageConsumerHandle::new(crate::core::BrokerConsumerAdapter::new(b))
    }
}
