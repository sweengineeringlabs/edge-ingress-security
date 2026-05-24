//! Integration tests for GrpcTimeoutParser::parse.

use std::time::Duration;
use swe_edge_ingress_grpc_transport::GrpcTimeoutParser;

/// @covers: GrpcTimeoutParser::parse
#[test]
fn test_parse_handles_all_six_unit_suffixes() {
    assert_eq!(
        GrpcTimeoutParser::parse("1H"),
        Some(Duration::from_secs(3600))
    );
    assert_eq!(
        GrpcTimeoutParser::parse("2M"),
        Some(Duration::from_secs(120))
    );
    assert_eq!(GrpcTimeoutParser::parse("5S"), Some(Duration::from_secs(5)));
    assert_eq!(
        GrpcTimeoutParser::parse("250m"),
        Some(Duration::from_millis(250))
    );
    assert_eq!(
        GrpcTimeoutParser::parse("100u"),
        Some(Duration::from_micros(100))
    );
    assert_eq!(
        GrpcTimeoutParser::parse("99999999n"),
        Some(Duration::from_nanos(99_999_999))
    );
}

/// @covers: GrpcTimeoutParser::parse
#[test]
fn test_parse_returns_none_for_malformed_input() {
    assert_eq!(GrpcTimeoutParser::parse(""), None, "empty");
    assert_eq!(GrpcTimeoutParser::parse("S"), None, "missing digits");
    assert_eq!(GrpcTimeoutParser::parse("123"), None, "missing unit");
    assert_eq!(GrpcTimeoutParser::parse("99X"), None, "unknown unit");
    assert_eq!(
        GrpcTimeoutParser::parse("123456789m"),
        None,
        "more than 8 digits"
    );
    assert_eq!(GrpcTimeoutParser::parse("12.3S"), None, "non-integer");
}

/// @covers: GrpcTimeoutParser::parse
#[test]
fn test_parse_zero_returns_zero_duration() {
    assert_eq!(GrpcTimeoutParser::parse("0n"), Some(Duration::ZERO));
    assert_eq!(GrpcTimeoutParser::parse("0S"), Some(Duration::ZERO));
}
