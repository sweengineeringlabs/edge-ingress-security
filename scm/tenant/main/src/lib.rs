//! `swe-edge-ingress-tenant` — tenant identity resolution for inbound HTTP requests.
//!
//! Extracts a [`TenantId`] from inbound request headers using one of four
//! strategies: JWT claim, named header, subdomain, or noop (single-tenant).
//!
//! ## Quick start
//!
//! ```rust
//! use swe_edge_ingress_tenant::{TenantResolverConfig, TenantSvc};
//!
//! let config = TenantResolverConfig::default();
//! assert_eq!(config.strategy, "noop");
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod saf;
mod spi;

pub use saf::*;
