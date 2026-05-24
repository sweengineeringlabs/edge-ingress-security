//! Integration tests for GrpcHealthCheck.

use swe_edge_ingress_grpc_transport::GrpcHealthCheck;

/// @covers: GrpcHealthCheck::healthy
#[test]
fn test_grpc_health_check_healthy_is_true() {
    let h = GrpcHealthCheck::healthy();
    assert!(h.healthy);
    assert!(h.message.is_none());
}

/// @covers: GrpcHealthCheck::unhealthy
#[test]
fn test_grpc_health_check_unhealthy_sets_message() {
    let h = GrpcHealthCheck::unhealthy("down");
    assert!(!h.healthy);
    assert_eq!(h.message.as_deref(), Some("down"));
}
