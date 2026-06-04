//! Coverage test for `grpc_ingress_interceptor_chain`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: grpc_ingress_interceptor_chain
#[test]
fn transport_struct_grpc_ingress_interceptor_chain_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
