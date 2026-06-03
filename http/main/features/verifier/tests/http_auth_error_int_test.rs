//! Tests for HttpAuthError.
use swe_edge_ingress_http_verifier::HttpAuthError;
/// @covers: HttpAuthError
#[test]
fn verifier_struct_http_auth_error_variants_are_accessible_int_test() {
    assert!(matches!(
        HttpAuthError::MissingAuthorization,
        HttpAuthError::MissingAuthorization
    ));
    assert!(matches!(
        HttpAuthError::MalformedAuthorization,
        HttpAuthError::MalformedAuthorization
    ));
}
