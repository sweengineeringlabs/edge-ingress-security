//! Audit sink — callback contract for per-dispatch audit events.

pub(crate) mod audit;
pub(crate) mod noop_audit_sink;

pub use audit::{AuditEvent, AuditEventBuilder, AuditSink};
pub use noop_audit_sink::NoopAuditSink;
