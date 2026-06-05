//! Integration tests — SAF public API for the ingress message consumer.

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_ingress_message_broker_transport::{
        check_health, default_consumer, subscribe_to, validate, ApplicationConfigBuilder,
        MessageConsumer, Validator,
    };

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_default_consumer_saf_factory_returns_healthy_consumer() {
        let c = default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_subscribe_to_returns_stream_for_default_consumer() {
        let c = default_consumer();
        assert!(subscribe_to(&c, "events.test").await.is_ok());
    }

    #[tokio::test]
    async fn test_check_health_returns_ok_for_default_consumer() {
        let c = default_consumer();
        assert!(check_health(&c).await.is_ok());
    }

    #[test]
    fn test_validate_returns_ok_for_always_valid() {
        assert!(validate(&AlwaysValid).is_ok());
    }

    #[test]
    fn test_application_config_builder_builds_with_custom_capacity() {
        let cfg = ApplicationConfigBuilder::new().with_capacity(512);
        assert_eq!(cfg.capacity, 512);
    }
}
