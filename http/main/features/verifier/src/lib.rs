//! `swe-edge-ingress-http-verifier` — tower/axum bearer-token authentication layer.
//!
//! Add [`BearerLayer`] to an axum router to enforce JWT authentication on all
//! routes.  Verified claims are available as `Extension<VerifiedClaims>` in
//! downstream handlers.

pub mod api;
pub(crate) mod core;
pub mod saf;
mod spi;

pub use saf::*;
