//! `swe-edge-ingress-grpc` — gRPC inbound domain (value objects + GrpcIngress port).

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
mod api;
mod core;
mod gateway;
mod saf;

pub use gateway::*;
