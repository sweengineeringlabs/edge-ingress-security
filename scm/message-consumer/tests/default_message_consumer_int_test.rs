//! Integration tests — broker consumer via SAF (replaces default/in-memory backend tests).
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

/// @covers: BrokerConsumerAdapter — subscribes via SAF
#[tokio::test]
async fn test_broker_consumer_adapter_subscribes_successfully() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.subscribe("default.test").await.is_ok());
}

/// @covers: BrokerConsumerAdapter — health check via SAF
#[tokio::test]
async fn test_broker_consumer_adapter_health_check_returns_ok() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.health_check().await.is_ok());
}
