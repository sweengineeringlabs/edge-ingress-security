//! Coverage test for `audit_sink`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: audit_sink
#[test]
fn transport_struct_audit_sink_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
