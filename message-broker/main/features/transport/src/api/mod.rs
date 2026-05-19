//! API layer — ingress message consumer port contracts.
pub(crate) mod application_config_builder;
pub(crate) mod default_message_consumer;
pub(crate) mod nats_message_consumer;
pub(crate) mod port;
pub(crate) mod traits;
pub(crate) mod validator;

pub use application_config_builder::ApplicationConfigBuilder;
pub use port::{ConsumerError, ConsumerResult, MessageConsumer};
pub use traits::Validator;
