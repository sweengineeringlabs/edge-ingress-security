//! Audit sink — callback contract for per-dispatch audit events.

pub(crate) mod audit;

pub use crate::api::types::audit::{AuditEvent, AuditEventBuilder, NoopAuditSink};
pub use audit::AuditSink;
