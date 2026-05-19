//! SAF factory functions for the ingress message consumer.

use futures::future::BoxFuture;
use swe_edge_message_broker::MessageStream;

use crate::api::port::consumer::consumer_error::ConsumerError;
use crate::api::port::consumer::consumer_result::ConsumerResult;
use crate::api::port::message_consumer::MessageConsumer;
use crate::api::traits::Validator;

/// Validate any type that implements [`Validator`].
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

/// Subscribe to `topic` using any [`MessageConsumer`].
pub fn subscribe_to<'a>(
    consumer: &'a dyn MessageConsumer,
    topic: &'a str,
) -> BoxFuture<'a, ConsumerResult<MessageStream>> {
    consumer.subscribe(topic)
}

/// Run a health check on any [`MessageConsumer`].
pub fn check_health(consumer: &dyn MessageConsumer) -> BoxFuture<'_, ConsumerResult<()>> {
    consumer.health_check()
}

/// Construct an in-memory consumer backed by a tokio broadcast channel.
///
/// Requires the `in-memory` feature.
#[cfg(feature = "in-memory")]
pub fn default_consumer() -> impl MessageConsumer + Clone {
    crate::core::DefaultMessageConsumer::new()
}

/// Connect to a NATS server and return a consumer handle.
///
/// # Errors
/// Returns [`ConsumerError::Connection`] when the server is unreachable.
///
/// Requires the `nats` feature.
#[cfg(feature = "nats")]
pub async fn nats_consumer(url: &str) -> Result<impl MessageConsumer + Clone, ConsumerError> {
    use swe_edge_message_broker::nats_broker;
    let broker = nats_broker(url).await.map_err(ConsumerError::from)?;
    Ok(crate::core::NatsMessageConsumer::new(broker))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::DefaultValidator;
    use futures::future::BoxFuture;
    use swe_edge_message_broker::MessageStream;

    struct NeverConsumer;
    impl MessageConsumer for NeverConsumer {
        fn subscribe<'a>(&'a self, _: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>> {
            Box::pin(futures::future::ready(Ok(
                Box::pin(futures::stream::empty()) as MessageStream,
            )))
        }
        fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    /// @covers: validate
    #[test]
    fn test_validate_returns_ok_for_default_validator() {
        assert!(validate(&DefaultValidator).is_ok());
    }

    /// @covers: subscribe_to
    #[test]
    fn test_subscribe_to_accepts_any_message_consumer() {
        let _fut = subscribe_to(&NeverConsumer, "t");
    }

    /// @covers: check_health
    #[test]
    fn test_check_health_accepts_any_message_consumer() {
        let _fut = check_health(&NeverConsumer);
    }

    /// @covers: default_consumer
    #[test]
    fn test_default_consumer_is_feature_gated_behind_in_memory() {
        let _enabled = cfg!(feature = "in-memory");
    }

    /// @covers: nats_consumer
    #[test]
    fn test_nats_consumer_is_feature_gated() {
        let _enabled = cfg!(feature = "nats");
    }

    #[tokio::test]
    async fn test_subscribe_to_returns_ok_for_never_consumer() {
        assert!(subscribe_to(&NeverConsumer, "t").await.is_ok());
    }

    #[tokio::test]
    async fn test_check_health_returns_ok_for_never_consumer() {
        assert!(check_health(&NeverConsumer).await.is_ok());
    }
}
