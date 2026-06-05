//! Integration tests — MessageConsumer port via SAF factories.

#![allow(clippy::expect_used, clippy::unwrap_used)]

#[cfg(feature = "in-memory")]
mod in_memory {
    use futures::StreamExt;
    use swe_edge_ingress_message_broker_transport::{MessageBrokerSvc, MessageConsumer};

    #[tokio::test]
    async fn test_default_consumer_health_check_returns_ok() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_default_consumer_subscribe_returns_stream() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(c.subscribe("test.topic").await.is_ok());
    }

    #[tokio::test]
    async fn test_default_consumer_stream_is_driveable() {
        let c = MessageBrokerSvc::default_consumer();
        let mut stream = c.subscribe("events.test").await.expect("subscribe ok");
        // No publisher — stream is empty. Verify it can be polled without panic.
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(10), stream.next()).await;
        // Timeout is expected — stream is live but empty.
        assert!(result.is_err() || result.expect("inner").is_none());
    }

    #[tokio::test]
    async fn test_default_consumer_clone_produces_independent_handle() {
        let c1 = MessageBrokerSvc::default_consumer();
        let c2 = c1.clone();
        assert!(c1.health_check().await.is_ok());
        assert!(c2.health_check().await.is_ok());
    }
}
