//! `swe-edge-ingress-http-verifier` — tower/axum bearer-token authentication layer.

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
