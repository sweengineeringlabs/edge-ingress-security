//! `swe-edge-ingress-message-consumer` — injection-only ingress message consumer port.
//!
//! Defines the [`MessageConsumer`] port contract. The assembler injects a concrete
//! backend via [`MessageConsumerSvc::from_broker`] or [`MessageConsumerSvc::consumer`].
//! This crate never constructs runtime brokers itself.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! swe-edge-ingress-message-consumer = { path = "..." }
//! ```
//!
//! ```rust,ignore
//! use swe_edge_ingress_message_consumer::{MessageConsumerSvc, MessageConsumer};
//! use futures::StreamExt;
//!
//! // The assembler constructs and injects the broker:
//! let consumer = MessageConsumerSvc::from_broker(broker);
//! let mut stream = consumer.subscribe("orders.created").await?;
//! while let Some(msg) = stream.next().await {
//!     // process msg
//! }
//! ```
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod gateway;
mod saf;

pub use gateway::*;
