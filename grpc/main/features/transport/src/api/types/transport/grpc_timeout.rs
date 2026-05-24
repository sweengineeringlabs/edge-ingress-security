//! gRPC timeout parser.

use std::time::Duration;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: parse_grpc_timeout
    #[test]
    fn test_parse_grpc_timeout_handles_all_six_unit_suffixes() {
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

    /// @covers: parse_grpc_timeout
    #[test]
    fn test_parse_grpc_timeout_returns_none_for_malformed_input() {
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

    /// @covers: parse_grpc_timeout
    #[test]
    fn test_parse_grpc_timeout_zero_returns_zero_duration() {
        assert_eq!(GrpcTimeoutParser::parse("0n"), Some(Duration::ZERO));
        assert_eq!(GrpcTimeoutParser::parse("0S"), Some(Duration::ZERO));
    }
}
