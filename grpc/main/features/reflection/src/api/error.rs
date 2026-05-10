//! Error types for the reflection crate.

use thiserror::Error;

/// Errors raised while encoding/decoding reflection wire messages.
///
/// The reflection RPC itself never returns these on the wire — it
/// translates them into a structured `ErrorResponse` so grpcurl can
/// keep its bidi stream open.  They are surfaced from the codec to
/// the dispatcher and are useful in tests.
#[derive(Debug, Error)]
pub enum ReflectionError {
    /// Wire-format error in an inbound `ServerReflectionRequest` —
    /// truncated body, malformed varint, etc.
    #[error("malformed reflection request: {0}")]
    Malformed(String),

    /// Inbound request was syntactically valid but its `message_request`
    /// oneof carried a field number this server does not understand.
    #[error("unknown reflection request field: {0}")]
    UnknownRequest(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: ReflectionError::Malformed
    #[test]
    fn test_malformed_displays_message() {
        let e = ReflectionError::Malformed("truncated body".into());
        assert!(e.to_string().contains("truncated body"));
    }

    /// @covers: ReflectionError::UnknownRequest
    #[test]
    fn test_unknown_request_displays_field_number() {
        let e = ReflectionError::UnknownRequest(99);
        assert!(e.to_string().contains("99"));
    }
}
