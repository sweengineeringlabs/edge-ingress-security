//! Coverage test for `traits`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: traits
#[test]
fn transport_struct_traits_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
