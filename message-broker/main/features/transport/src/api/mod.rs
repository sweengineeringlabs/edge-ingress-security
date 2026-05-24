//! API layer — ingress message consumer port contracts.
pub(crate) mod default_message_consumer;
pub(crate) mod message_consumer_config;
pub(crate) mod nats_message_consumer;
pub(crate) mod port;
pub(crate) mod traits;
pub(crate) mod validator;

pub use message_consumer_config::MessageConsumerConfig;
pub use port::{ConsumerError, ConsumerResult, MessageConsumer};
pub use traits::Validator;
