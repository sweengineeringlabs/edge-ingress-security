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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: healthy
    #[test]
    fn test_grpc_health_check_healthy_is_true() {
        let h = GrpcHealthCheck::healthy();
        assert!(h.healthy);
        assert!(h.message.is_none());
    }

    /// @covers: unhealthy
    #[test]
    fn test_grpc_health_check_unhealthy_sets_message() {
        let h = GrpcHealthCheck::unhealthy("down");
        assert!(!h.healthy);
        assert_eq!(h.message.as_deref(), Some("down"));
    }
}
