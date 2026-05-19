//! `swe-edge-ingress-message-broker` — opt-in ingress message consumer port.
//!
//! Wraps `swe-edge-message-broker` as a structured ingress port. Nothing is
//! compiled unless the caller opts in via a feature flag.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! swe-edge-ingress-message-broker = { path = "...", features = ["in-memory"] }
//! ```
//!
//! ```rust,ignore
//! use swe_edge_ingress_message_broker::{default_consumer, MessageConsumer};
//! use futures::StreamExt;
//!
//! let consumer = default_consumer();
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
