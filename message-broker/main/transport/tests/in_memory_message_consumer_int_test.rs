//! Integration tests — InMemoryMessageConsumer interface contract.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_ingress_message_broker_transport::{MessageBrokerSvc, MessageConsumer};

    /// @covers: InMemoryMessageConsumer — satisfied by DefaultMessageConsumer via SAF
    #[tokio::test]
    async fn test_in_memory_consumer_satisfies_message_consumer_contract() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(c.subscribe("test.topic").await.is_ok());
        assert!(c.health_check().await.is_ok());
    }
}
