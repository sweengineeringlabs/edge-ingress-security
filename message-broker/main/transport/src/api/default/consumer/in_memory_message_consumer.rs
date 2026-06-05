//! Interface specification for the in-memory (default) message consumer backend.
//!
//! The concrete implementation in `core/default/consumer/` must implement
//! [`crate::api::port::MessageConsumer`] to satisfy this interface contract.

/// Marker trait for in-memory (default) message consumer implementations.
///
/// The core consumer implements this and a compile-time assertion in
/// `core/default/consumer/` bounds on it, so it is a live part of the contract.
pub trait InMemoryMessageConsumer: crate::api::port::MessageConsumer {}
