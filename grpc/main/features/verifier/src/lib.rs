//! `swe-edge-ingress-grpc-verifier` — bearer-token auth interceptor for gRPC inbound calls.
//!
//! Provides [`BearerTokenInterceptor`] which implements both
//! [`GrpcIngressInterceptor`](swe_edge_ingress_grpc_transport::GrpcIngressInterceptor)
//! and [`AuthorizationInterceptor`](swe_edge_ingress_grpc_transport::AuthorizationInterceptor).
//!
//! Wire it into a [`GrpcIngressInterceptorChain`](swe_edge_ingress_grpc_transport::GrpcIngressInterceptorChain)
//! to gate all inbound gRPC calls on a valid JWT Bearer token.

pub mod api;
pub(crate) mod core;
pub mod saf;

pub use saf::*;
