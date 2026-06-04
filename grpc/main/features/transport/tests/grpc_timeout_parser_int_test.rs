//! Integration tests for `GrpcTimeoutParser`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_grpc_transport::GrpcTimeoutParser;

/// @covers: GrpcTimeoutParser::parse
#[test]
fn transport_struct_grpc_timeout_parser_parse_valid_seconds_int_test() {
    let d = GrpcTimeoutParser::parse("30S");
    assert!(d.is_some());
}

/// @covers: GrpcTimeoutParser::parse
#[test]
fn transport_struct_grpc_timeout_parser_parse_invalid_returns_none_int_test() {
    let d = GrpcTimeoutParser::parse("");
    assert!(d.is_none());
}
