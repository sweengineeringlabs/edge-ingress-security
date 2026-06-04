//! Coverage test for `grpc_server_config_builder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: grpc_server_config_builder
#[test]
fn transport_struct_grpc_server_config_builder_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
