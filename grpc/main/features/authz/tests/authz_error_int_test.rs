//! Integration tests for [`AuthzError`].
//!
//! @covers: fmt

use swe_edge_ingress_grpc_authz::*;

/// @covers: fmt
#[test]
fn test_no_identity_display_mentions_ordering_invariant() {
    assert!(AuthzError::NoIdentity
        .to_string()
        .contains("authz must run after authn"));
}

/// @covers: fmt
#[test]
fn test_denied_display_does_not_leak_method_or_identity() {
    let s = AuthzError::Denied.to_string();
    assert_eq!(s, "authorization denied");
}
