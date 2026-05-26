//! Audit sink — callback contract for per-dispatch audit events.

#[path = "audit/audit_sink.rs"]
pub(crate) mod audit_sink;

pub use crate::api::types::audit::{AuditEvent, AuditEventBuilder, NoopAuditSink};
pub use audit_sink::AuditSink;
