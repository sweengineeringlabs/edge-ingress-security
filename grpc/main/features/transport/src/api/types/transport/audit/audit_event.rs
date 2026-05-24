//! Single audit event emitted once per dispatched gRPC call.

use std::time::SystemTime;

use crate::api::value_object::GrpcStatusCode;

/// A single audit event emitted once per dispatched gRPC call.
///
/// The fields are intentionally minimal — sinks that need richer
/// context can pull it off [`crate::GrpcRequest::metadata`] from the
/// surrounding interceptor chain and inject it into their own pipeline.
#[derive(Debug, Clone)]
pub struct AuditEvent {
    /// Timestamp captured at dispatch time.
    pub timestamp: SystemTime,
    /// Fully-qualified gRPC method path
    /// (e.g. `"/pkg.Service/Method"`).
    pub method: String,
    /// Caller identity — `None` when the request was accepted under
    /// `allow_unauthenticated = true`.  Otherwise carries the
    /// fully-qualified principal name set by the authn / authz chain.
    pub identity: Option<String>,
    /// Final gRPC status code returned to the wire.
    pub status: GrpcStatusCode,
    /// Wall-clock duration of the dispatch in milliseconds.
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

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
}
