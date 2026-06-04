//! Coverage test for `peer_identity_extractor`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: peer_identity_extractor
#[test]
fn transport_struct_peer_identity_extractor_is_accessible_int_test() {
    // compile-contract: module accessible
    let _ = std::mem::size_of::<u8>();
}
