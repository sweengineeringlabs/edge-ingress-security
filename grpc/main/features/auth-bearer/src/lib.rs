//! `swe-edge-ingress-grpc-auth-bearer` — JWT bearer
//! [`GrpcInboundInterceptor`] for the gRPC ingress stack.
//!
//! [`BearerInboundInterceptor`] validates incoming
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

mod api;
mod core;
mod saf;

pub use saf::*;
