//! `swe-edge-ingress-grpc-auth-bearer` — JWT bearer
//! [`GrpcIngressInterceptor`] for the gRPC ingress stack.
//!
//! [`BearerIngressInterceptor`] validates incoming
//! `authorization: Bearer <jwt>` against a configured secret/key,
//! then surfaces the verified `sub` claim under the internal metadata
//! key [`crate::EXTRACTED_BEARER_SUBJECT`] for downstream authz
//! policies — and **only** after successful verification.
//!
//! For the outbound (client-side) bearer injection counterpart see
//! `swe-edge-egress-grpc-auth-bearer`.
//!
//! Constant-time comparisons (`subtle`) are used for any symmetric
//! shared-secret material in the configuration loaders.
//!
//! See `docs/threat_model.md` for the STRIDE breakdown.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
