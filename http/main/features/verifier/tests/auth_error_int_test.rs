//! Tests for HttpAuthError.

use swe_edge_ingress_http_verifier::HttpAuthError;

#[test]
fn test_http_auth_error_missing_authorization() {
    let err = HttpAuthError::MissingAuthorization;
    let msg = err.to_string();
    assert!(msg.contains("Authorization"));
}

#[test]
fn test_http_auth_error_malformed_authorization() {
    let err = HttpAuthError::MalformedAuthorization;
    let msg = err.to_string();
    assert!(msg.contains("Bearer"));
}
