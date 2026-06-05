//! Interface specification for the broker-adapter message consumer implementation.
//!
//! The concrete implementation in `core/broker/consumer/` must implement
//! [`crate::api::traits::MessageConsumer`] to satisfy this interface contract.

/// Marker trait for broker-adapter message consumer implementations.
///
/// The core consumer implements this and a compile-time `PhantomData` reference
/// in `core/broker/consumer/` names it in a type position, so it is a live part
/// of the contract.
pub trait BrokerMessageConsumer: crate::api::traits::MessageConsumer {}
