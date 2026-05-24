//! Integration tests for AuditEvent.

use std::time::SystemTime;
use swe_edge_ingress_grpc_transport::{AuditEvent, GrpcStatusCode};

/// @covers: AuditEvent
#[test]
fn test_audit_event_fields_are_accessible() {
    let evt = AuditEvent {
        timestamp: SystemTime::UNIX_EPOCH,
        method: "/svc/M".into(),
        identity: Some("alice".into()),
        status: GrpcStatusCode::Ok,
        duration_ms: 42,
    };
    assert_eq!(evt.method, "/svc/M");
    assert_eq!(evt.duration_ms, 42);
}
