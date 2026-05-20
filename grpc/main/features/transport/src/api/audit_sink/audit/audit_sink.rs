//! Receiver of audit events.

use super::audit_event::AuditEvent;

/// Receiver of [`AuditEvent`]s.
///
/// Implementations MUST be cheap and non-blocking — the server calls
/// `record` on the dispatch path; a slow sink will back up request
/// processing.  Real implementations typically push the event onto an
/// in-memory channel and let a background task drain it.
pub trait AuditSink: Send + Sync {
    /// Record one audit event.  Implementations must not panic.
    fn record(&self, event: AuditEvent);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::api::value_object::GrpcStatusCode;

    use super::*;
    use crate::api::audit_sink::audit::audit_event::AuditEvent;

    /// Capturing sink — records every event into a shared `Vec` for assertions.
    struct CapturingAuditSink {
        events: Arc<Mutex<Vec<AuditEvent>>>,
    }

    impl AuditSink for CapturingAuditSink {
        fn record(&self, event: AuditEvent) {
            self.events.lock().unwrap().push(event);
        }
    }

    #[test]
    fn test_audit_sink_trait_is_object_safe() {
        fn _accept(_: &dyn AuditSink) {}
    }

    #[test]
    fn test_capturing_sink_record_receives_every_event() {
        use std::time::SystemTime;
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink = CapturingAuditSink {
            events: events.clone(),
        };
        sink.record(AuditEvent {
            timestamp: SystemTime::UNIX_EPOCH,
            method: "/svc/A".into(),
            identity: Some("alice".into()),
            status: GrpcStatusCode::Ok,
            duration_ms: 5,
        });
        sink.record(AuditEvent {
            timestamp: SystemTime::UNIX_EPOCH,
            method: "/svc/B".into(),
            identity: None,
            status: GrpcStatusCode::PermissionDenied,
            duration_ms: 7,
        });
        let captured = events.lock().unwrap().clone();
        assert_eq!(captured.len(), 2);
        assert_eq!(captured[0].method, "/svc/A");
        assert_eq!(captured[0].status, GrpcStatusCode::Ok);
        assert_eq!(captured[1].method, "/svc/B");
        assert_eq!(captured[1].status, GrpcStatusCode::PermissionDenied);
        assert_eq!(captured[1].identity, None);
    }
}
