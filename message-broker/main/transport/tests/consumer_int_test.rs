//! Integration tests — MessageConsumer port via SAF factories.

#[cfg(feature = "in-memory")]
mod tests {
    use futures::StreamExt;
    use swe_edge_ingress_message_broker_transport::{default_consumer, MessageConsumer};

    #[tokio::test]
    async fn test_default_consumer_health_check_returns_ok() {
        let c = default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_default_consumer_subscribe_returns_stream() {
        let c = default_consumer();
        assert!(c.subscribe("test.topic").await.is_ok());
    }

    #[tokio::test]
    async fn test_default_consumer_stream_is_driveable() {
        let c = default_consumer();
        let mut stream = c.subscribe("events.test").await.unwrap();
        // No publisher — stream is empty. Verify it can be polled without panic.
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(10), stream.next()).await;
        // Timeout is expected — stream is live but empty.
        assert!(result.is_err() || result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_default_consumer_clone_produces_independent_handle() {
        let c1 = default_consumer();
        let c2 = c1.clone();
        assert!(c1.health_check().await.is_ok());
        assert!(c2.health_check().await.is_ok());
    }
}
