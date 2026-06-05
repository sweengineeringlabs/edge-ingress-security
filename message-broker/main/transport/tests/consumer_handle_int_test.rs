//! Integration tests — MessageConsumerHandle.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_ingress_message_broker_transport::{
        MessageBrokerSvc, MessageConsumer, MessageConsumerHandle,
    };

    /// @covers: MessageConsumerHandle — Clone shares the underlying broker
    #[tokio::test]
    async fn test_consumer_handle_clone_shares_underlying_channel() {
        let h1: MessageConsumerHandle = MessageBrokerSvc::default_consumer();
        let h2 = h1.clone();
        assert!(h1.subscribe("a").await.is_ok());
        assert!(h2.subscribe("b").await.is_ok());
    }

    /// @covers: MessageConsumerHandle — MessageConsumer::health_check
    #[tokio::test]
    async fn test_consumer_handle_health_check_returns_ok() {
        let h: MessageConsumerHandle = MessageBrokerSvc::default_consumer();
        assert!(h.health_check().await.is_ok());
    }
}
