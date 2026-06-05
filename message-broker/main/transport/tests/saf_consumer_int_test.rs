//! Integration tests — SAF public API for the ingress message consumer.
//!
//! Covers rules 125 (SAF pub fn must have API-level tests) and 77 (all pub fns tested).

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_ingress_message_broker_transport::{
        ApplicationConfigBuilder, MessageBrokerSvc, MessageConsumer, Validator,
    };

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_default_consumer_saf_factory_returns_healthy_consumer() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_subscribe_to_returns_stream_for_default_consumer() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(MessageBrokerSvc::subscribe_to(&c, "events.test")
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_check_health_returns_ok_for_in_memory_consumer() {
        let c = MessageBrokerSvc::default_consumer();
        assert!(MessageBrokerSvc::check_health(&c).await.is_ok());
    }

    #[test]
    fn test_validate_returns_ok_for_always_valid() {
        assert!(MessageBrokerSvc::validate(&AlwaysValid).is_ok());
    }

    #[test]
    fn test_application_config_builder_builds_with_custom_capacity() {
        let cfg = ApplicationConfigBuilder::new().with_capacity(512);
        assert_eq!(cfg.capacity, 512);
    }
}
