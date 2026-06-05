//! Integration tests — default (in-memory) message consumer.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_ingress_message_broker_transport::{MessageBrokerSvc, MessageConsumer};

    /// @covers: DefaultMessageConsumer — subscribes via SAF
    #[tokio::test]
    async fn test_default_message_consumer_subscribes_successfully() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(c.subscribe("default.test").await.is_ok());
    }

    /// @covers: DefaultMessageConsumer — health check via SAF
    #[tokio::test]
    async fn test_default_message_consumer_health_check_returns_ok() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }
}
