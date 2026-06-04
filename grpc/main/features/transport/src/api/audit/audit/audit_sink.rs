//! Receiver of audit events.

use crate::api::types::audit::AuditEvent;

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
