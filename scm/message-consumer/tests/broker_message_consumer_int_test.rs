//! Integration tests — BrokerMessageConsumer trait and BrokerConsumerAdapter.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use futures::StreamExt as _;
use swe_edge_ingress_message_consumer::{MessageConsumer, MessageConsumerSvc};
use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};

/// Minimal in-memory broker for integration tests.
struct TestBroker;

impl MessageBroker for TestBroker {
    fn publish<'a>(
        &'a self,
        _topic: &'a str,
        _msg: Message,
    ) -> futures::future::BoxFuture<'a, Result<(), BrokerError>> {
        Box::pin(futures::future::ready(Ok(())))
    }

    fn subscribe<'a>(
        &'a self,
        _topic: &'a str,
    ) -> futures::future::BoxFuture<'a, Result<MessageStream, BrokerError>> {
        Box::pin(futures::future::ready(Ok(
            Box::pin(futures::stream::empty()) as MessageStream,
        )))
    }

    fn health_check(&self) -> futures::future::BoxFuture<'_, Result<(), BrokerError>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

/// @covers: BrokerConsumerAdapter — adapts a MessageBroker to the MessageConsumer port
/// @covers: BrokerMessageConsumer — concrete impl satisfies the marker trait
#[tokio::test]
async fn test_from_broker_creates_consumer_that_health_checks_ok() {
    let handle = MessageConsumerSvc::from_broker(TestBroker);
    assert!(handle.health_check().await.is_ok());
}

/// @covers: BrokerConsumerAdapter — subscribe returns a stream
#[tokio::test]
async fn test_from_broker_subscribe_to_topic_returns_empty_stream() {
    let handle = MessageConsumerSvc::from_broker(TestBroker);
    let mut stream = handle
        .subscribe("orders.created")
        .await
        .expect("subscribe must succeed");
    assert!(
        stream.next().await.is_none(),
        "TestBroker emits no messages"
    );
}

/// @covers: BrokerConsumerAdapter — trait object safety: consumer handle is a dyn MessageConsumer
#[test]
fn test_message_consumer_handle_is_object_safe() {
    fn _assert(_: &dyn MessageConsumer) {}
}
