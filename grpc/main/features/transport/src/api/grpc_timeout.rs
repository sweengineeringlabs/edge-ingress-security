//! gRPC timeout — `DEFAULT_DEADLINE` constant and `GrpcTimeoutParser`.

use std::time::Duration;

/// Server-side default deadline applied when the client did not send a
/// `grpc-timeout` header.
pub const DEFAULT_DEADLINE: Duration = Duration::from_secs(30);

/// Parser for `grpc-timeout` header values.
pub struct GrpcTimeoutParser;

impl GrpcTimeoutParser {
    /// Parse a `grpc-timeout` header value.  Returns `Some(d)` on success,
    /// `None` for malformed inputs (caller should fall back to the default).
    pub fn parse(value: &str) -> Option<Duration> {
        if value.is_empty() {
            return None;
        }
        let bytes = value.as_bytes();
        let unit_byte = bytes[bytes.len() - 1];
        let digits = &value[..value.len() - 1];
        if digits.is_empty() || digits.len() > 8 {
            return None;
        }
        let n: u64 = digits.parse().ok()?;
        Some(match unit_byte {
            b'H' => Duration::from_secs(n.checked_mul(3600)?),
            b'M' => Duration::from_secs(n.checked_mul(60)?),
            b'S' => Duration::from_secs(n),
            b'm' => Duration::from_millis(n),
            b'u' => Duration::from_micros(n),
            b'n' => Duration::from_nanos(n),
            _ => return None,
        })
    }
}

/// Backward-compatibility wrapper for parse function.
pub fn parse_grpc_timeout(value: &str) -> Option<Duration> {
    GrpcTimeoutParser::parse(value)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
