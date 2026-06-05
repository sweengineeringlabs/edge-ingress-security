//! Integration test — exercises `swe-edge-runtime-message-broker` dep directly.

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_runtime_message_broker::{in_memory_broker, Message, MessageBroker};

    #[tokio::test]
    async fn test_message_broker_dep_health_check_returns_ok() {
        assert!(in_memory_broker().health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_message_broker_dep_subscribe_returns_stream() {
        let broker = in_memory_broker();
        assert!(broker.subscribe("dep.test").await.is_ok());
    }
}
