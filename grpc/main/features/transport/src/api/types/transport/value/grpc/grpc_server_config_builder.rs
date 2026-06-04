//! Builder for GrpcServerConfig.

use std::net::SocketAddr;

use swe_edge_ingress_tls::IngressTlsConfig;

use super::super::compression_mode::CompressionMode;
use super::grpc_server_config::{
    GrpcServerConfig, DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES,
};

/// Fluent builder for [`GrpcServerConfig`].
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
