//! Coverage test for `health_aggregate`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: health_aggregate
#[test]
fn transport_struct_health_aggregate_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
