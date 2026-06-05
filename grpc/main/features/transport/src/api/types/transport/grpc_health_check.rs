//! Minimal health-check result for the gRPC domain.

/// Minimal health-check result for the gRPC domain.
#[derive(Debug, Clone)]
pub struct GrpcHealthCheck {
    /// `true` when the handler is healthy.
    pub healthy: bool,
    /// Optional human-readable status detail.
    pub message: Option<String>,
}

impl GrpcHealthCheck {
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
