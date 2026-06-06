//! `swe-edge-ingress-verifier` — protocol-agnostic inbound credential verification.
//!
//! Provides [`JwtVerifier`] (HS256/RS256/ES256), [`ApiKeyVerifier`] (constant-time),
//! and the [`TokenVerifier`] trait for pluggable bearer-token checks.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
