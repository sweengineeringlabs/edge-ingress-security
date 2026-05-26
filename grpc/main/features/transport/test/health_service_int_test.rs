//! Integration tests for the `HealthService` SAF surface.

/// @covers: HealthService — accessible via SAF and constructible.
#[test]
fn test_health_service_is_constructible_via_saf() {
    use swe_edge_ingress_grpc_transport::HealthService;
    let _s = HealthService::new();
}
