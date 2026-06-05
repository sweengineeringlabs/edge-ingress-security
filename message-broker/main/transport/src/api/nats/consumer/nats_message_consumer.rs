//! Interface specification for the NATS message consumer implementation.
//!
//! The concrete type in `core/nats/consumer/nats_message_consumer.rs`
//! implements [`crate::api::port::MessageConsumer`] to satisfy this contract.

/// Marker trait for NATS-backed message consumer implementations.
///
/// The core consumer implements this and a compile-time assertion in
/// `core/nats/consumer/` bounds on it, so it is a live part of the contract.
pub trait NatsMessageConsumer: crate::api::port::MessageConsumer {}
