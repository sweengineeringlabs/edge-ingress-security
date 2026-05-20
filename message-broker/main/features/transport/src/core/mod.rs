//! Core implementations — pub(crate) only.
#[cfg(feature = "in-memory")]
pub(crate) mod default_message_consumer;
#[cfg(feature = "nats")]
pub(crate) mod nats_message_consumer;
pub(crate) mod validator;

#[cfg(feature = "in-memory")]
pub(crate) use default_message_consumer::DefaultMessageConsumer;
#[cfg(feature = "nats")]
pub(crate) use nats_message_consumer::NatsMessageConsumer;
pub(crate) use validator::DefaultValidator;
