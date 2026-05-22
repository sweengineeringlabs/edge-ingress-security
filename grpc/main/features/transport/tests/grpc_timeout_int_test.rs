//! Integration tests for parse_grpc_timeout.

use std::time::Duration;
use swe_edge_ingress_grpc_transport::parse_grpc_timeout;

/// @covers: parse_grpc_timeout
#[test]
fn test_parse_grpc_timeout_handles_all_six_unit_suffixes() {
    assert_eq!(parse_grpc_timeout("1H"), Some(Duration::from_secs(3600)));
    assert_eq!(parse_grpc_timeout("2M"), Some(Duration::from_secs(120)));
    assert_eq!(parse_grpc_timeout("5S"), Some(Duration::from_secs(5)));
    assert_eq!(parse_grpc_timeout("250m"), Some(Duration::from_millis(250)));
    assert_eq!(parse_grpc_timeout("100u"), Some(Duration::from_micros(100)));
    assert_eq!(
        parse_grpc_timeout("99999999n"),
        Some(Duration::from_nanos(99_999_999))
    );
}

/// @covers: parse_grpc_timeout
#[test]
fn test_parse_grpc_timeout_returns_none_for_malformed_input() {
    assert_eq!(parse_grpc_timeout(""), None, "empty");
    assert_eq!(parse_grpc_timeout("S"), None, "missing digits");
    assert_eq!(parse_grpc_timeout("123"), None, "missing unit");
    assert_eq!(parse_grpc_timeout("99X"), None, "unknown unit");
    assert_eq!(parse_grpc_timeout("123456789m"), None, "more than 8 digits");
    assert_eq!(parse_grpc_timeout("12.3S"), None, "non-integer");
}

/// @covers: parse_grpc_timeout
#[test]
fn test_parse_grpc_timeout_zero_returns_zero_duration() {
    assert_eq!(parse_grpc_timeout("0n"), Some(Duration::ZERO));
    assert_eq!(parse_grpc_timeout("0S"), Some(Duration::ZERO));
}
