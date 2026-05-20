//! Builder for GrpcServerConfig.

use std::net::SocketAddr;

use swe_edge_ingress_tls::IngressTlsConfig;

use super::super::compression_mode::CompressionMode;
use super::grpc_server_config::{
    GrpcServerConfig, DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES,
};

/// Fluent builder for [`GrpcServerConfig`].
#[allow(dead_code)]
pub struct GrpcServerConfigBuilder {
    bind: SocketAddr,
    tls_required: bool,
    tls: Option<IngressTlsConfig>,
    max_message_bytes: usize,
    max_concurrent_streams: u32,
    allow_unauthenticated: bool,
    compression: CompressionMode,
    enable_reflection: bool,
}

impl GrpcServerConfigBuilder {
    /// Start building a configuration bound to `bind` with TLS required by default.
    pub fn new(bind: SocketAddr) -> Self {
        Self {
            bind,
            tls_required: true,
            tls: None,
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            max_concurrent_streams: DEFAULT_MAX_CONCURRENT_STREAMS,
            allow_unauthenticated: false,
            compression: CompressionMode::None,
            enable_reflection: false,
        }
    }

    /// Relax the TLS requirement to allow plaintext connections.
    pub fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }

    /// Attach TLS configuration.
    pub fn with_tls(mut self, tls: IngressTlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// Override the max message bytes cap.
    pub fn with_max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    /// Override the max concurrent streams cap.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    /// Allow unauthenticated callers.
    pub fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    /// Set the compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Enable gRPC reflection.
    pub fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }

    /// Build the [`GrpcServerConfig`].
    pub fn build(self) -> GrpcServerConfig {
        GrpcServerConfig {
            bind: self.bind,
            tls_required: self.tls_required,
            tls: self.tls,
            max_message_bytes: self.max_message_bytes,
            max_concurrent_streams: self.max_concurrent_streams,
            allow_unauthenticated: self.allow_unauthenticated,
            compression: self.compression,
            enable_reflection: self.enable_reflection,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr() -> SocketAddr {
        "127.0.0.1:0".parse().unwrap()
    }

    #[test]
    fn test_new_creates_builder_with_tls_required_true() {
        let b = GrpcServerConfigBuilder::new(addr());
        assert!(b.tls_required);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_grpc_server_config() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .build();
        assert!(!cfg.tls_required);
    }

    /// @covers: allow_plaintext
    #[test]
    fn test_allow_plaintext_sets_tls_required_false() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .build();
        assert!(!cfg.tls_required);
    }

    /// @covers: with_max_message_bytes
    #[test]
    fn test_with_max_message_bytes_overrides_default() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .with_max_message_bytes(1024)
            .build();
        assert_eq!(cfg.max_message_bytes, 1024);
    }

    /// @covers: with_max_concurrent_streams
    #[test]
    fn test_with_max_concurrent_streams_overrides_default() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .with_max_concurrent_streams(50)
            .build();
        assert_eq!(cfg.max_concurrent_streams, 50);
    }

    /// @covers: enable_reflection
    #[test]
    fn test_enable_reflection_sets_reflection_flag() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .enable_reflection()
            .build();
        assert!(cfg.enable_reflection);
    }

    /// @covers: with_compression
    #[test]
    fn test_with_compression_sets_compression_mode() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .with_compression(CompressionMode::Gzip)
            .build();
        assert!(matches!(cfg.compression, CompressionMode::Gzip));
    }

    /// @covers: allow_unauthenticated
    #[test]
    fn test_allow_unauthenticated_sets_the_flag() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .allow_plaintext()
            .allow_unauthenticated()
            .build();
        assert!(cfg.allow_unauthenticated);
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_sets_tls_config() {
        let cfg = GrpcServerConfigBuilder::new(addr())
            .with_tls(IngressTlsConfig::tls("c.pem", "k.pem"))
            .build();
        assert!(cfg.tls.is_some());
    }
}
