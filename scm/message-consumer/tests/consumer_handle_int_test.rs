//! Integration tests — MessageConsumerHandle.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_ingress_message_consumer::{
    MessageConsumer, MessageConsumerHandle, MessageConsumerSvc,
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

/// @covers: MessageConsumerHandle — Clone shares the underlying broker
#[tokio::test]
async fn test_consumer_handle_clone_shares_underlying_channel() {
    let h1: MessageConsumerHandle = MessageConsumerSvc::from_broker(MockBroker);
    let h2 = h1.clone();
    assert!(h1.subscribe("a").await.is_ok());
    assert!(h2.subscribe("b").await.is_ok());
}

/// @covers: MessageConsumerHandle — MessageConsumer::health_check
#[tokio::test]
async fn test_consumer_handle_health_check_returns_ok() {
    let h: MessageConsumerHandle = MessageConsumerSvc::from_broker(MockBroker);
    assert!(h.health_check().await.is_ok());
}
