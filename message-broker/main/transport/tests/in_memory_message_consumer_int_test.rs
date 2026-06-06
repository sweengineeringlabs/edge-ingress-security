//! Integration tests — BrokerConsumerAdapter satisfies the MessageConsumer contract.
//!
//! Previously tested the in-memory backend; now tests via injected mock broker.

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

/// @covers: BrokerConsumerAdapter — satisfied by from_broker via SAF
#[tokio::test]
async fn test_broker_consumer_satisfies_message_consumer_contract() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.subscribe("test.topic").await.is_ok());
    assert!(c.health_check().await.is_ok());
}
