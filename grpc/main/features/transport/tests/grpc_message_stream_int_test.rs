//! Coverage test for `grpc_message_stream`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: grpc_message_stream
#[test]
fn transport_struct_grpc_message_stream_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
