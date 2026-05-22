//! Integration tests for GrpcStatusCode.

use swe_edge_ingress_grpc_transport::GrpcStatusCode;

/// @covers: GrpcStatusCode
#[test]
fn test_grpc_status_code_has_17_distinct_variants() {
    let codes = [
        GrpcStatusCode::Ok,
        GrpcStatusCode::Cancelled,
        GrpcStatusCode::Unknown,
        GrpcStatusCode::InvalidArgument,
        GrpcStatusCode::DeadlineExceeded,
        GrpcStatusCode::NotFound,
        GrpcStatusCode::AlreadyExists,
        GrpcStatusCode::PermissionDenied,
        GrpcStatusCode::ResourceExhausted,
        GrpcStatusCode::FailedPrecondition,
        GrpcStatusCode::Aborted,
        GrpcStatusCode::OutOfRange,
        GrpcStatusCode::Unimplemented,
        GrpcStatusCode::Internal,
        GrpcStatusCode::Unavailable,
        GrpcStatusCode::DataLoss,
        GrpcStatusCode::Unauthenticated,
    ];
    assert_eq!(codes.len(), 17);
    assert_eq!(GrpcStatusCode::Ok, GrpcStatusCode::Ok);
    assert_ne!(GrpcStatusCode::Ok, GrpcStatusCode::Internal);
}
