//! Integration tests — MessageConsumerHandle type.

use swe_edge_ingress_message_consumer::{
    MessageConsumerSvc, MessageConsumer, MessageConsumerHandle,
};
use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};

struct MockBroker;
impl MessageBroker for MockBroker {
    fn publish<'a>(&'a self, _: &'a str, _: Message) -> futures::future::BoxFuture<'a, Result<(), BrokerError>> {
        Box::pin(futures::future::ready(Ok(())))
    }
    fn subscribe<'a>(&'a self, _: &'a str) -> futures::future::BoxFuture<'a, Result<MessageStream, BrokerError>> {
        Box::pin(futures::future::ready(Ok(Box::pin(futures::stream::empty()) as MessageStream)))
    }
    fn health_check(&self) -> futures::future::BoxFuture<'_, Result<(), BrokerError>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

/// @covers: MessageConsumerHandle — wraps consumer via MessageConsumerSvc::from_broker
#[tokio::test]
async fn test_message_consumer_handle_wraps_broker_consumer() {
    let h: MessageConsumerHandle = MessageConsumerSvc::from_broker(MockBroker);
    assert!(h.health_check().await.is_ok());
}
