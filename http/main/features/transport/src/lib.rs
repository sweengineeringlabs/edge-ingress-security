//! `swe-edge-ingress-http` — HTTP inbound domain (value objects + HttpIngress port).
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
