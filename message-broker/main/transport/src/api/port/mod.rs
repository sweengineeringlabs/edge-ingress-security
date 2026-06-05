//! Ingress message consumer port traits and error types.
pub(crate) mod consumer;
pub(crate) mod message_consumer;

pub use consumer::{ConsumerError, ConsumerResult};
pub use message_consumer::MessageConsumer;
