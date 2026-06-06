//! Integration tests — SAF public API for the ingress message consumer.
//!
//! Covers rules 125 (SAF pub fn must have API-level tests) and 77 (all pub fns tested).

use swe_edge_ingress_message_consumer::{
    ApplicationConfigBuilder, MessageConsumer, MessageConsumerSvc, Validator,
};
use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};

struct MockBroker;
impl MessageBroker for MockBroker {
    fn publish<'a>(
        &'a self,
        _: &'a str,
        _: Message,
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

struct AlwaysValid;
impl Validator for AlwaysValid {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn test_from_broker_saf_factory_returns_healthy_consumer() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.health_check().await.is_ok());
}

#[tokio::test]
async fn test_subscribe_to_returns_stream_for_broker_consumer() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(MessageConsumerSvc::subscribe_to(&c, "events.test")
        .await
        .is_ok());
}

#[tokio::test]
async fn test_check_health_returns_ok_for_broker_consumer() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(MessageConsumerSvc::check_health(&c).await.is_ok());
}

#[test]
fn test_validate_returns_ok_for_always_valid() {
    assert!(MessageConsumerSvc::validate(&AlwaysValid).is_ok());
}

#[test]
fn test_application_config_builder_builds_with_custom_capacity() {
    let cfg = ApplicationConfigBuilder::new().with_capacity(512);
    assert_eq!(cfg.capacity, 512);
}
