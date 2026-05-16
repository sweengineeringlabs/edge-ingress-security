//! HTTP inbound trait — handles incoming HTTP requests.

use edge_domain::RequestContext;
use futures::future::BoxFuture;

use crate::api::value_object::{HttpRequest, HttpResponse};

/// Result type for HTTP inbound operations.
pub type HttpInboundResult<T> = Result<T, HttpInboundError>;

/// Error type for HTTP inbound operations.
#[derive(Debug, thiserror::Error)]
pub enum HttpInboundError {
    /// Internal server error.
    #[error("internal: {0}")]
    Internal(String),
    /// Requested resource not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// Request input failed validation.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Upstream service unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// Operation timed out.
    #[error("timeout: {0}")]
    Timeout(String),
    /// Caller is not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// Caller lacks permission.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// The operation conflicts with existing state.
    #[error("conflict: {0}")]
    Conflict(String),
    /// The handler does not support the requested operation. Maps to HTTP 405.
    #[error("method not allowed: {0}")]
    MethodNotAllowed(String),
    /// The request is valid but rejected by a business rule or precondition. Maps to HTTP 422.
    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),
}

/// Minimal health-check result for the HTTP domain.
#[derive(Debug, Clone)]
pub struct HttpHealthCheck {
    /// `true` when the handler is healthy.
    pub healthy: bool,
    /// Optional human-readable status detail.
    pub message: Option<String>,
}

impl HttpHealthCheck {
    /// Create a healthy result.
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            message: None,
        }
    }
    /// Create an unhealthy result with a message.
    pub fn unhealthy(msg: impl Into<String>) -> Self {
        Self {
            healthy: false,
            message: Some(msg.into()),
        }
    }
}

/// Receives and handles inbound HTTP requests.
pub trait HttpInbound: Send + Sync {
    /// Handle an HTTP request with its per-request context and return a response.
    ///
    /// `ctx` carries the authenticated identity, tenant, and trace metadata
    /// extracted by the ingress middleware stack before dispatch.
    fn handle(
        &self,
        request: HttpRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>>;
    /// Perform a health check of this handler.
    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_inbound_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpInbound) {}
    }

    #[test]
    fn test_http_inbound_error_internal_formats_correctly() {
        let err = HttpInboundError::Internal("oops".into());
        assert!(err.to_string().contains("oops"));
    }

    #[test]
    fn test_http_health_check_healthy_is_true() {
        let h = HttpHealthCheck::healthy();
        assert!(h.healthy);
        assert!(h.message.is_none());
    }

    #[test]
    fn test_http_health_check_unhealthy_sets_message() {
        let h = HttpHealthCheck::unhealthy("down");
        assert!(!h.healthy);
        assert_eq!(h.message.as_deref(), Some("down"));
    }
}
