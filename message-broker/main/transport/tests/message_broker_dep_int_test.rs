//! Integration test — exercises the `swe-edge-runtime-message-broker` backend directly.

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_message_broker::MessageBroker;
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    #[tokio::test]
    async fn test_message_broker_dep_health_check_returns_ok() {
        assert!(MessageBrokerFactory::in_memory()
            .health_check()
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_message_broker_dep_subscribe_returns_stream() {
        let broker = MessageBrokerFactory::in_memory();
        assert!(broker.subscribe("dep.test").await.is_ok());
    }
}
