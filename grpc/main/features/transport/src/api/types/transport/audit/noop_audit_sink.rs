//! Default no-op audit sink.

use super::audit_event::AuditEvent;
use crate::api::audit::AuditSink;

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
    fn record(&self, _event: AuditEvent) {
        // No-op: events are intentionally dropped.
        let _ = _event;
    }
}

impl Default for NoopAuditSink {
    fn default() -> Self {
        Self
    }
}
