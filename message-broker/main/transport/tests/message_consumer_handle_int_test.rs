//! Integration tests — MessageConsumerHandle type.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_ingress_message_broker_transport::{
        MessageBrokerSvc, MessageConsumer, MessageConsumerHandle,
    };

    /// @covers: MessageConsumerHandle — wraps consumer via MessageBrokerSvc
    #[tokio::test]
    async fn test_message_consumer_handle_wraps_default_consumer() {
        let h: MessageConsumerHandle = MessageBrokerSvc::default_consumer();
        assert!(h.health_check().await.is_ok());
    }
}
