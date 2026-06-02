//! `swe-edge-ingress-http` — HTTP inbound domain (value objects + HttpIngress port).

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
