//! Dependency coverage tests for verifier.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_grpc_transport::GrpcIngressInterceptorChain;
use swe_edge_ingress_verifier::TokenVerifier;

/// @covers: swe-edge-ingress-grpc-transport
#[test]
fn verifier_struct_grpc_ingress_interceptor_chain_dep_is_linked_int_test() {
    let _ = std::any::type_name::<GrpcIngressInterceptorChain>();
}

/// @covers: swe-edge-ingress-verifier
#[test]
fn verifier_trait_token_verifier_dep_is_linked_int_test() {
    let _ = std::any::type_name::<dyn TokenVerifier>();
}
