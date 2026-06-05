//! Ingress message consumer port trait and result type.
pub(crate) mod consumer_result;
pub(crate) mod message_consumer;

pub use consumer_result::ConsumerResult;
pub use message_consumer::MessageConsumer;
