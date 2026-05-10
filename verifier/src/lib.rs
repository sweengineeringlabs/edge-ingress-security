//! `swe-edge-ingress-verifier` — protocol-agnostic inbound credential verification.
//!
//! Provides [`JwtVerifier`] (HS256/RS256/ES256), [`ApiKeyVerifier`] (constant-time),
//! and the [`TokenVerifier`] trait for pluggable bearer-token checks.

mod api;
pub(crate) mod core;
pub mod saf;

pub use saf::*;
