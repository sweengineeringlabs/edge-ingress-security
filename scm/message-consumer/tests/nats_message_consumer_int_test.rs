//! Integration tests — BrokerConsumerAdapter with injected broker (replaces NATS feature-gated tests).
//!
//! The NATS backend is a runtime concern wired by the assembler. This test verifies
//! the from_broker() injection path using a mock broker.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_ingress_message_consumer::{MessageConsumer, MessageConsumerSvc};
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

/// @covers: BrokerConsumerAdapter — injection-based consumer is healthy
#[tokio::test]
async fn test_broker_consumer_adapter_from_injected_broker_is_healthy() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.health_check().await.is_ok());
}

/// @covers: BrokerConsumerAdapter — injection-based consumer can subscribe
#[tokio::test]
async fn test_broker_consumer_adapter_from_injected_broker_can_subscribe() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.subscribe("nats.replaced").await.is_ok());
}
