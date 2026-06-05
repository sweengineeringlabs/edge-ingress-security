//! Integration tests — MessageConsumer trait contract.

use futures::future::BoxFuture;
use swe_edge_ingress_message_broker_transport::{ConsumerResult, MessageConsumer, MessageStream};

struct NeverConsumer;
impl MessageConsumer for NeverConsumer {
    fn subscribe<'a>(&'a self, _: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>> {
        Box::pin(futures::future::ready(Ok(
            Box::pin(futures::stream::empty()) as MessageStream,
        )))
    }
    fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

/// @covers: MessageConsumer — object safety
#[test]
fn test_message_consumer_is_object_safe() {
    fn _assert(_: &dyn MessageConsumer) {}
}

/// @covers: MessageConsumer::health_check
#[tokio::test]
async fn test_message_consumer_health_check_returns_ok() {
    assert!(NeverConsumer.health_check().await.is_ok());
}

/// @covers: MessageConsumer::subscribe
#[tokio::test]
async fn test_message_consumer_subscribe_returns_ok() {
    assert!(NeverConsumer.subscribe("t").await.is_ok());
}
