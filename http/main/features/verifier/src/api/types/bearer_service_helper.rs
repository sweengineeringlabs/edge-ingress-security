//! Helper functions for BearerService implementation.

use crate::api::error::HttpAuthError;
use axum::body::Body;
use axum::http::{Request, Response, StatusCode};

/// Helper struct for bearer token service operations.
pub struct BearerServiceHelper;

impl BearerServiceHelper {
    /// Extract bearer token from Authorization header.
    pub fn extract_bearer(req: &Request<Body>) -> Result<&str, HttpAuthError> {
        let header = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(HttpAuthError::MissingAuthorization)?
            .to_str()
            .map_err(|_| HttpAuthError::MalformedAuthorization)?;

        header
            .strip_prefix("Bearer ")
            .ok_or(HttpAuthError::MalformedAuthorization)
    }

    /// Build 401 response for auth errors.
    pub fn auth_error_response(err: HttpAuthError) -> Response<Body> {
        tracing::debug!(?err, "bearer auth rejected request");
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from(err.to_string()))
            .expect("response with known-good headers and status cannot fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_extract_bearer_returns_token_from_valid_header() {
        let req = Request::builder()
            .header("Authorization", "Bearer valid-token")
            .body(Body::empty())
            .unwrap();
        let token = BearerServiceHelper::extract_bearer(&req).unwrap();
        assert_eq!(token, "valid-token");
    }

    #[test]
    fn test_extract_bearer_returns_error_for_missing_header() {
        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            BearerServiceHelper::extract_bearer(&req),
            Err(HttpAuthError::MissingAuthorization)
        ));
    }

    #[test]
    fn test_extract_bearer_returns_error_for_malformed_header() {
        let req = Request::builder()
            .header("Authorization", "Basic dXNlcjpwYXNz")
            .body(Body::empty())
            .unwrap();
        assert!(matches!(
            BearerServiceHelper::extract_bearer(&req),
            Err(HttpAuthError::MalformedAuthorization)
        ));
    }

    #[test]
    fn test_auth_error_response_returns_401() {
        let resp = BearerServiceHelper::auth_error_response(HttpAuthError::MissingAuthorization);
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
