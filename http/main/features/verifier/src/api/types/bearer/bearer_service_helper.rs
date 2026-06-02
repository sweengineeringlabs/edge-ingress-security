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
        let mut response = Response::new(Body::from(err.to_string()));
        *response.status_mut() = StatusCode::UNAUTHORIZED;
        response
    }
}
