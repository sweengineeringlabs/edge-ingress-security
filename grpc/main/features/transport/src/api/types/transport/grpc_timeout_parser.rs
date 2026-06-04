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
