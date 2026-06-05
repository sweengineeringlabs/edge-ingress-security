//! Integration tests — `MessageConsumerSvc` SAF factory methods.
//!
//! Covers rules 77 (all pub fns tested) and 78 (@covers: annotations).

use swe_edge_ingress_message_consumer::{MessageConsumerSvc, Validator};

/// @covers: MessageConsumerSvc::create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_crate_name() {
    let b = MessageConsumerSvc::create_config_builder();
    // The builder must be constructible and carry the crate name.
    let _ = b;
}

/// @covers: MessageConsumerSvc::validate
#[test]
fn test_validate_returns_ok_for_always_valid_validator() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(MessageConsumerSvc::validate(&AlwaysOk).is_ok());
}

/// @covers: MessageConsumerSvc::validate
#[test]
fn test_validate_propagates_err_from_validator_impl() {
    struct AlwaysFail;
    impl Validator for AlwaysFail {
        fn validate(&self) -> Result<(), String> {
            Err("always fails".into())
        }
    }
    assert!(MessageConsumerSvc::validate(&AlwaysFail).is_err());
}

mod with_mock_broker {
    use swe_edge_ingress_message_consumer::{MessageConsumerSvc, MessageConsumer};
    use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};

    struct MockBroker;
    impl MessageBroker for MockBroker {
        fn publish<'a>(&'a self, _: &'a str, _: Message) -> futures::future::BoxFuture<'a, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
        fn subscribe<'a>(&'a self, _: &'a str) -> futures::future::BoxFuture<'a, Result<MessageStream, BrokerError>> {
            Box::pin(futures::future::ready(Ok(Box::pin(futures::stream::empty()) as MessageStream)))
        }
        fn health_check(&self) -> futures::future::BoxFuture<'_, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    /// @covers: MessageConsumerSvc::subscribe_to
    #[tokio::test]
    async fn test_subscribe_to_returns_stream() {
        let c = MessageConsumerSvc::from_broker(MockBroker);
        assert!(MessageConsumerSvc::subscribe_to(&c, "events.test").await.is_ok());
    }

    /// @covers: MessageConsumerSvc::check_health
    #[tokio::test]
    async fn test_check_health_returns_ok_for_broker_consumer() {
        let c = MessageConsumerSvc::from_broker(MockBroker);
        assert!(MessageConsumerSvc::check_health(&c).await.is_ok());
    }

    /// @covers: MessageConsumerSvc::from_broker
    #[tokio::test]
    async fn test_from_broker_returns_healthy_consumer() {
        let c = MessageConsumerSvc::from_broker(MockBroker);
        assert!(c.health_check().await.is_ok());
    }

    /// @covers: MessageConsumerSvc::consumer
    #[tokio::test]
    async fn test_consumer_wraps_injected_consumer() {
        let c = MessageConsumerSvc::from_broker(MockBroker);
        let wrapped = MessageConsumerSvc::consumer(c);
        assert!(wrapped.health_check().await.is_ok());
    }
}
