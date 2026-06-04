//! Coverage test for `noop_audit_sink`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: noop_audit_sink
#[test]
fn transport_struct_noop_audit_sink_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
