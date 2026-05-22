//! `swe-edge-ingress-grpc` — gRPC inbound domain (value objects + GrpcIngress port).
#![allow(dead_code)]

mod api;
mod core;
mod saf;

pub use saf::*;
