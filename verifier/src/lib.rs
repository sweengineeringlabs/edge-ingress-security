//! `swe-edge-ingress-verifier` — protocol-agnostic inbound credential verification.
//!
//! Provides [`JwtVerifier`] (HS256/RS256/ES256), [`ApiKeyVerifier`] (constant-time),
//! and the [`TokenVerifier`] trait for pluggable bearer-token checks.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use crate::api::traits::TokenVerifier;
pub use gateway::*;
