//! Audit event and sink types.

pub(crate) mod audit_sink;

pub use crate::api::types::audit::{AuditEvent, AuditEventBuilder};
pub use audit_sink::AuditSink;
