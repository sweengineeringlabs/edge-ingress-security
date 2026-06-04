//! Compression negotiation modes for gRPC servers.

use serde::{Deserialize, Serialize};

/// Wire-level compression scheme accepted by the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CompressionMode {
    /// No compression accepted — identity only.
    #[default]
    None,
    /// gzip compression accepted.
    Gzip,
    /// zstandard compression accepted.
    Zstd,
}

impl CompressionMode {
    /// `grpc-accept-encoding` advertised value, or `None` for identity-only.
    pub fn header_value(self) -> Option<&'static str> {
        match self {
            CompressionMode::None => None,
            CompressionMode::Gzip => Some("gzip"),
            CompressionMode::Zstd => Some("zstd"),
        }
    }
}

