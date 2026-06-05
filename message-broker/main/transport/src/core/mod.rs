//! Core implementations — pub(crate) only.
#[cfg(feature = "in-memory")]
pub(crate) mod default;
#[cfg(feature = "nats")]
pub(crate) mod nats;

#[cfg(feature = "in-memory")]
pub(crate) use default::DefaultMessageConsumer;
#[cfg(feature = "nats")]
pub(crate) use nats::NatsMessageConsumer;
