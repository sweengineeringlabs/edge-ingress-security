//! WebSocket message value object.

use bytes::Bytes;

/// A single WebSocket message frame.
#[derive(Debug, Clone)]
pub struct WsMessage {
    /// Raw payload bytes.
    pub data: Bytes,
    /// `true` for binary frames; `false` for UTF-8 text frames.
    pub binary: bool,
}

impl WsMessage {
    /// Construct a text frame from a UTF-8 string.
    pub fn text(data: impl Into<String>) -> Self {
        Self {
            data: Bytes::from(data.into().into_bytes()),
            binary: false,
        }
    }

    /// Construct a binary frame.
    pub fn binary(data: impl Into<Bytes>) -> Self {
        Self {
            data: data.into(),
            binary: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: text
    #[test]
    fn test_ws_message_text_sets_binary_false() {
        let m = WsMessage::text("hello");
        assert!(!m.binary);
        assert_eq!(m.data.as_ref(), b"hello");
    }

    /// @covers: binary
    #[test]
    fn test_ws_message_binary_sets_binary_true() {
        let m = WsMessage::binary(vec![1u8, 2, 3]);
        assert!(m.binary);
        assert_eq!(m.data.as_ref(), &[1, 2, 3]);
    }
}
