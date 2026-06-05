//! Builder for AuditEvent.

use std::time::SystemTime;

use crate::api::error::GrpcIngressError;
use crate::api::types::GrpcIngressResult;
use crate::api::value::GrpcStatusCode;

use super::audit_event::AuditEvent;

/// Fluent builder for [`AuditEvent`].
#[derive(Default)]
pub struct AuditEventBuilder {
    timestamp: Option<SystemTime>,
    method: Option<String>,
    identity: Option<String>,
    status: Option<GrpcStatusCode>,
    duration_ms: Option<u64>,
}

impl AuditEventBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the dispatch timestamp.
    pub fn timestamp(mut self, ts: SystemTime) -> Self {
        self.timestamp = Some(ts);
        self
    }

    /// Set the gRPC method path.
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Set the caller identity (or `None` for unauthenticated calls).
    pub fn identity(mut self, id: impl Into<String>) -> Self {
        self.identity = Some(id.into());
        self
    }

    /// Set the final gRPC status code.
    pub fn status(mut self, status: GrpcStatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the wall-clock dispatch duration in milliseconds.
    pub fn duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = Some(ms);
        self
    }

    /// Build the [`AuditEvent`].
    ///
    /// Returns [`GrpcIngressError::InvalidArgument`] if `method` or `status`
    /// was not set.
    pub fn build(self) -> GrpcIngressResult<AuditEvent> {
        let Some(method) = self.method else {
            return Err(GrpcIngressError::InvalidArgument(
                "method is required".into(),
            ));
        };
        let Some(status) = self.status else {
            return Err(GrpcIngressError::InvalidArgument(
                "status is required".into(),
            ));
        };

        Ok(AuditEvent {
            timestamp: self.timestamp.unwrap_or_else(SystemTime::now),
            method,
            identity: self.identity,
            status,
            duration_ms: self.duration_ms.unwrap_or(0),
        })
    }
}
