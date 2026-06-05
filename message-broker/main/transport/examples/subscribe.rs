//! Example — subscribe to a topic using the in-memory backend.
//!
//! Run with:
//! ```bash
//! cargo run --example subscribe --features in-memory
//! ```

// Examples favour terse `.expect()` over production-grade error handling.
#![allow(clippy::expect_used, clippy::unwrap_used)]

#[cfg(feature = "in-memory")]
#[tokio::main]
async fn main() {
    use futures::StreamExt;
    use swe_edge_ingress_message_broker_transport::MessageBrokerSvc;

    let consumer = MessageBrokerSvc::default_consumer();
    let mut stream = MessageBrokerSvc::subscribe_to(&consumer, "example.topic")
        .await
        .expect("subscribe failed");
    println!("Subscribed to example.topic; waiting for messages...");

    // No publisher in this example — poll once with a timeout to show liveness.
    match tokio::time::timeout(std::time::Duration::from_millis(50), stream.next()).await {
        Ok(Some(_msg)) => println!("Received a message."),
        Ok(None) => println!("Stream closed."),
        Err(_) => println!("No message within timeout (expected — no publisher)."),
    }
}

#[cfg(not(feature = "in-memory"))]
fn main() {
    eprintln!("Enable the `in-memory` feature to run this example.");
}
