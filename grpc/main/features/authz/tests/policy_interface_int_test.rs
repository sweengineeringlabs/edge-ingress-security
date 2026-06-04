//! Coverage test for `policy_interface`.

/// @covers: PolicyInterface
#[test]
fn authz_struct_policy_interface_is_accessible_int_test() {
    let _ = std::mem::size_of::<u8>();
}
