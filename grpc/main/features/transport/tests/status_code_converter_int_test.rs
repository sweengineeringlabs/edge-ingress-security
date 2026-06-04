//! Integration tests for `StatusCodeConverter`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_grpc_transport::StatusCodeConverter;

/// @covers: StatusCodeConverter::from_tonic_code
#[test]
fn transport_struct_status_code_converter_from_tonic_code_ok_int_test() {
    let _ = StatusCodeConverter::from_tonic_code(tonic::Code::Ok);
}

/// @covers: StatusCodeConverter::to_tonic_code
#[test]
fn transport_struct_status_code_converter_to_tonic_code_ok_int_test() {
    use swe_edge_ingress_grpc_transport::GrpcStatusCode;
    let code = StatusCodeConverter::to_tonic_code(GrpcStatusCode::Ok);
    assert_eq!(code, tonic::Code::Ok);
}
