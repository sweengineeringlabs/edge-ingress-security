//! `swe-edge-ingress-grpc-verifier` — bearer-token auth interceptor for gRPC inbound calls.
//!
//! Provides [`BearerTokenInterceptor`] which implements both
//! [`GrpcIngressInterceptor`](swe_edge_ingress_grpc_transport::GrpcIngressInterceptor)
//! and [`AuthorizationInterceptor`](swe_edge_ingress_grpc_transport::AuthorizationInterceptor).
//!
//! Wire it into a [`GrpcIngressInterceptorChain`](swe_edge_ingress_grpc_transport::GrpcIngressInterceptorChain)
//! to gate all inbound gRPC calls on a valid JWT Bearer token.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
