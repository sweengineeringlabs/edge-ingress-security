//! Example — subscribe to a topic using an injected mock broker.
//!
//! Run with:
//! ```bash
//! cargo run --example subscribe
//! ```

// Examples favour terse `.expect()` over production-grade error handling.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use futures::StreamExt;
use swe_edge_ingress_message_consumer::MessageConsumerSvc;
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

#[tokio::main]
async fn main() {
    // The assembler injects the broker. Here we use a mock for illustration.
    let consumer = MessageConsumerSvc::from_broker(MockBroker);
    let mut stream = MessageConsumerSvc::subscribe_to(&consumer, "example.topic")
        .await
        .expect("subscribe failed");
    println!("Subscribed to example.topic; waiting for messages...");

    // Mock broker returns an empty stream — shows that the port contract works.
    match tokio::time::timeout(std::time::Duration::from_millis(10), stream.next()).await {
        Ok(Some(_msg)) => println!("Received a message."),
        Ok(None) => println!("Stream closed (mock broker returns empty stream)."),
        Err(_) => println!("No message within timeout."),
    }
}
