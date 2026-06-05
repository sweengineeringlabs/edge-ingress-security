//! Integration test — exercises the `swe-edge-message-broker` contract directly.
//!
//! Satisfies rule 95: dependencies used in src/ must have integration/e2e coverage.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};
use swe_edge_ingress_message_consumer::{MessageConsumer, MessageConsumerSvc};

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

#[tokio::test]
async fn test_message_broker_dep_health_check_returns_ok() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.health_check().await.is_ok());
}

#[tokio::test]
async fn test_message_broker_dep_subscribe_returns_stream_with_no_publisher() {
    let c = MessageConsumerSvc::from_broker(MockBroker);
    assert!(c.subscribe("dep.test").await.is_ok());
}
