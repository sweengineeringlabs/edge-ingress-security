//! Default no-op audit sink.

use super::audit_event::AuditEvent;
use crate::api::audit_sink::audit::audit_sink::AuditSink;

/// Default no-op sink — drops every event.
///
/// Used by [`crate::TonicGrpcServer::new`] when the caller does not
/// supply an explicit sink.  Production deployments should replace
/// this with a real implementation.
pub struct NoopAuditSink;

impl AuditSink for NoopAuditSink {
    /// Drop every audit event silently. This is the default sink for servers
    /// that do not configure explicit audit logging. Production deployments
    /// should replace this with a real `AuditSink` implementation.
    fn record(&self, _event: AuditEvent) {}
}

impl Default for NoopAuditSink {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use crate::api::value::GrpcStatusCode;

    use super::*;
    use crate::api::types::audit::AuditEvent;

    #[test]
    fn test_noop_audit_sink_record_drops_events_silently() {
        let sink = NoopAuditSink;
        sink.record(AuditEvent {
            timestamp: SystemTime::now(),
            method: "/svc/M".into(),
            identity: Some("alice".into()),
            status: GrpcStatusCode::Ok,
            duration_ms: 1,
        });
    }
}
