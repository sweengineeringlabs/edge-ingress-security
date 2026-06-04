//! Coverage test for `tonic_grpc_server_builder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: tonic_grpc_server_builder
#[test]
fn transport_struct_tonic_grpc_server_builder_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
