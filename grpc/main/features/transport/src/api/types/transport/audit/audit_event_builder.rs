//! Builder for AuditEvent.

use std::time::SystemTime;

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
    /// Panics if `method` or `status` was not set.
    pub fn build(self) -> AuditEvent {
        AuditEvent {
            timestamp: self.timestamp.unwrap_or_else(SystemTime::now),
            method: self.method.expect("method is required"),
            identity: self.identity,
            status: self.status.expect("status is required"),
            duration_ms: self.duration_ms.unwrap_or(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_builder() {
        let b = AuditEventBuilder::new();
        assert!(b.method.is_none());
    }

    /// @covers: build
    #[test]
    fn test_build_creates_audit_event_with_set_fields() {
        let evt = AuditEventBuilder::new()
            .method("/svc/M")
            .status(GrpcStatusCode::Ok)
            .duration_ms(5)
            .build();
        assert_eq!(evt.method, "/svc/M");
        assert_eq!(evt.status, GrpcStatusCode::Ok);
        assert_eq!(evt.duration_ms, 5);
        assert!(evt.identity.is_none());
    }

    /// @covers: identity
    #[test]
    fn test_identity_sets_caller_identity() {
        let evt = AuditEventBuilder::new()
            .method("/svc/M")
            .status(GrpcStatusCode::Ok)
            .identity("alice")
            .build();
        assert_eq!(evt.identity, Some("alice".into()));
    }

    /// @covers: timestamp
    #[test]
    fn test_timestamp_overrides_default() {
        let ts = std::time::SystemTime::UNIX_EPOCH;
        let evt = AuditEventBuilder::new()
            .method("/svc/M")
            .status(GrpcStatusCode::Ok)
            .timestamp(ts)
            .build();
        assert_eq!(evt.timestamp, ts);
    }

    /// @covers: duration_ms
    #[test]
    fn test_duration_ms_sets_duration() {
        let evt = AuditEventBuilder::new()
            .method("/svc/M")
            .status(GrpcStatusCode::Ok)
            .duration_ms(100)
            .build();
        assert_eq!(evt.duration_ms, 100);
    }

    /// @covers: method
    #[test]
    fn test_method_sets_grpc_method_path() {
        let evt = AuditEventBuilder::new()
            .method("/pkg.Svc/Call")
            .status(GrpcStatusCode::Ok)
            .build();
        assert_eq!(evt.method, "/pkg.Svc/Call");
    }

    /// @covers: status
    #[test]
    fn test_status_sets_grpc_status_code() {
        let evt = AuditEventBuilder::new()
            .method("/svc/M")
            .status(GrpcStatusCode::NotFound)
            .build();
        assert_eq!(evt.status, GrpcStatusCode::NotFound);
    }
}
