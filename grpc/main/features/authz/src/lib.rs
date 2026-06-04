//! `swe-edge-ingress-grpc-authz` — pluggable authorisation
//! [`GrpcIngressInterceptor`].
//!
//! The crate ships a small [`AuthzPolicy`] trait and a generic
//! [`AuthzInterceptor`] that wraps any policy.  The default
//! built-in policy is method-based RBAC keyed on the verified
//! peer identity already present in [`GrpcMetadata`] under the
//! reserved `x-edge-peer-cn` / `x-edge-extracted-bearer-subject`
//! keys (set upstream by the mTLS / bearer interceptors).
//!
//! Custom policies plug in by implementing [`AuthzPolicy`]; see
//! `docs/threat_model.md` for extension guidance.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
