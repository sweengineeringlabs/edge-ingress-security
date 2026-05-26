//! `swe-edge-ingress-http` — HTTP inbound domain (value objects + HttpIngress port).
#![allow(dead_code)]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::*;
