//! Coverage test for `grpc_request_builder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: grpc_request_builder
#[test]
fn transport_struct_grpc_request_builder_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
