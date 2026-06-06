//! Integration tests — MessageConsumer port via SAF factories.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use futures::StreamExt;
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

#[tokio::test]
async fn test_from_broker_consumer_health_check_returns_ok() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.health_check().await.is_ok());
}

#[tokio::test]
async fn test_from_broker_consumer_subscribe_returns_stream() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.subscribe("test.topic").await.is_ok());
}

#[tokio::test]
async fn test_from_broker_consumer_stream_is_driveable() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    let mut stream = c.subscribe("events.test").await.expect("subscribe ok");
    // Mock broker returns empty stream. Verify it can be polled without panic.
    let item = stream.next().await;
    assert!(item.is_none());
}

#[tokio::test]
async fn test_from_broker_consumer_clone_produces_independent_handle() {
    let c1 = MessageConsumerSvc::from_broker(MockBroker);
    let c2 = c1.clone();
    assert!(c1.health_check().await.is_ok());
    assert!(c2.health_check().await.is_ok());
}
