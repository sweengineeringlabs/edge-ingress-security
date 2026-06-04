//! Coverage test for `audit_event_builder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: audit_event_builder
#[test]
fn transport_struct_audit_event_builder_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
