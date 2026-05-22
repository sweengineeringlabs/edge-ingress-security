//! Integration tests for ServingStatus.

use swe_edge_ingress_grpc_transport::ServingStatus;

/// @covers: ServingStatus
#[test]
fn test_serving_status_wire_values_are_correct() {
    assert_eq!(ServingStatus::Unknown as i32, 0);
    assert_eq!(ServingStatus::Serving as i32, 1);
    assert_eq!(ServingStatus::NotServing as i32, 2);
    assert_eq!(ServingStatus::ServiceUnknown as i32, 3);
}
