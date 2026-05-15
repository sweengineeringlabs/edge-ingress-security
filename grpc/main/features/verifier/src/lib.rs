//! `swe-edge-ingress-grpc-verifier` — bearer-token auth interceptor for gRPC inbound calls.
//!
//! Provides [`BearerTokenInterceptor`] which implements both
//! [`GrpcInboundInterceptor`](swe_edge_ingress_grpc_transport::GrpcInboundInterceptor)
//! and [`AuthorizationInterceptor`](swe_edge_ingress_grpc_transport::AuthorizationInterceptor).
//!
//! Wire it into a [`GrpcInboundInterceptorChain`](swe_edge_ingress_grpc_transport::GrpcInboundInterceptorChain)
//! to gate all inbound gRPC calls on a valid JWT Bearer token.

pub mod api;
pub(crate) mod core;
pub mod saf;

pub use saf::*;

