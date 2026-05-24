//! Integration tests for [`BearerAuthError`].

use swe_edge_ingress_grpc_auth_bearer::*;

/// @covers: BearerAuthError
#[test]
fn test_bearer_auth_error_implements_std_error() {
    let e = BearerAuthError::MissingHeader;
    let _: &dyn std::error::Error = &e;
    assert!(e.to_string().contains("missing"));
}

/// @covers: BearerAuthError
#[test]
fn test_malformed_header_display_message() {
    let e = BearerAuthError::MalformedHeader;
    assert!(e.to_string().contains("malformed"));
}
