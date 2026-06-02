//! Tests for HttpAuthError.
use swe_edge_ingress_http_verifier::HttpAuthError;
/// @covers: HttpAuthError
#[test]
fn verifier_struct_http_auth_error_variants_are_accessible_int_test() {
    let e = HttpAuthError::MissingToken;
    assert!(matches!(e, HttpAuthError::MissingToken));
}
