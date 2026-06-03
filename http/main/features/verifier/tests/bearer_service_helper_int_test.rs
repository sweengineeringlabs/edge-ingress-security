//! Integration tests for `BearerServiceHelper` — bearer-token extraction and
//! the 401 error response it builds for the bearer authentication layer.

use axum::body::Body;
use axum::http::{header::AUTHORIZATION, HeaderValue, Request, StatusCode};
use swe_edge_ingress_http_verifier::{BearerServiceHelper, HttpAuthError};

fn request_with_auth(value: &'static str) -> Request<Body> {
    let mut req = Request::new(Body::empty());
    req.headers_mut()
        .insert(AUTHORIZATION, HeaderValue::from_static(value));
    req
}

/// @covers: extract_bearer
#[test]
fn verifier_struct_bearer_service_helper_extract_bearer_returns_token_int_test() {
    let req = request_with_auth("Bearer abc.def.ghi");
    assert_eq!(
        BearerServiceHelper::extract_bearer(&req).ok(),
        Some("abc.def.ghi")
    );
}

/// @covers: extract_bearer
#[test]
fn verifier_struct_bearer_service_helper_extract_bearer_missing_header_int_test() {
    let req = Request::new(Body::empty());
    assert!(matches!(
        BearerServiceHelper::extract_bearer(&req),
        Err(HttpAuthError::MissingAuthorization)
    ));
}

/// @covers: extract_bearer
#[test]
fn verifier_struct_bearer_service_helper_extract_bearer_malformed_int_test() {
    let req = request_with_auth("Basic abc.def.ghi");
    assert!(matches!(
        BearerServiceHelper::extract_bearer(&req),
        Err(HttpAuthError::MalformedAuthorization)
    ));
}

/// @covers: auth_error_response
#[test]
fn verifier_struct_bearer_service_helper_auth_error_response_sets_401_int_test() {
    let response = BearerServiceHelper::auth_error_response(HttpAuthError::MissingAuthorization);
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
