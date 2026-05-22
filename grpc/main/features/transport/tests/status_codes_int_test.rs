//! Integration tests for status code conversion functions.

use swe_edge_ingress_grpc_transport::{
    from_tonic_code, from_wire, map_inbound_error, to_tonic_code, to_wire, GrpcIngressError,
    GrpcStatusCode, SANITIZED_INTERNAL_MSG,
};

const ALL_17: [GrpcStatusCode; 17] = [
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

/// @covers: from_tonic_code
#[test]
fn test_from_tonic_code_round_trips_all_17_variants() {
    for code in ALL_17 {
        let tonic = to_tonic_code(code);
        assert_eq!(from_tonic_code(tonic), code);
    }
}

/// @covers: to_tonic_code
#[test]
fn test_to_tonic_code_round_trips_all_17_variants() {
    for code in ALL_17 {
        let tonic = to_tonic_code(code);
        assert_eq!(from_tonic_code(tonic), code);
    }
}

/// @covers: to_wire
#[test]
fn test_to_wire_round_trips_all_17_variants() {
    for code in ALL_17 {
        assert_eq!(from_wire(to_wire(code)), code);
    }
}

/// @covers: from_wire
#[test]
fn test_from_wire_round_trips_all_17_variants() {
    for code in ALL_17 {
        assert_eq!(from_wire(to_wire(code)), code);
    }
}

/// @covers: map_inbound_error
#[test]
fn test_map_inbound_error_internal_returns_sanitized_message() {
    let (code, msg) = map_inbound_error(GrpcIngressError::Internal("secret/path".into()));
    assert_eq!(code, tonic::Code::Internal);
    assert_eq!(msg, SANITIZED_INTERNAL_MSG);
}
