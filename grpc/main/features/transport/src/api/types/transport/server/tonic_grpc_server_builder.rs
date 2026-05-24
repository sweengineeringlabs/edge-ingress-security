//! Builder for TonicGrpcServer.

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;

use super::tonic_grpc_server::TonicGrpcServer;
use crate::api::audit_sink::AuditSink;
use crate::api::interceptor::GrpcIngressInterceptorChain;
use crate::api::port::grpc_ingress::GrpcIngress;
use crate::api::value::CompressionMode;

/// Fluent builder for [`TonicGrpcServer`].
///
/// This builder mirrors the `with_*` methods on `TonicGrpcServer` but aggregates
/// the configuration in one place before constructing the server.
pub struct TonicGrpcServerBuilder {
    bind: String,
    handler: Arc<dyn GrpcIngress>,
    max_bytes: Option<usize>,
    max_concurrent_streams: Option<u32>,
    tls: Option<IngressTlsConfig>,
    interceptors: Option<GrpcIngressInterceptorChain>,
    compression: Option<CompressionMode>,
    allow_unauthenticated: bool,
    audit_sink: Option<Arc<dyn AuditSink>>,
    enable_reflection: bool,
}

impl TonicGrpcServerBuilder {
    /// Start building a server bound to `bind` that delegates to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            max_bytes: None,
            max_concurrent_streams: None,
            tls: None,
            interceptors: None,
            compression: None,
            allow_unauthenticated: false,
            audit_sink: None,
            enable_reflection: false,
        }
    }

    /// Override the maximum inbound message size in bytes.
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_bytes = Some(size);
        self
    }

    /// Override the maximum number of concurrent HTTP/2 streams.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = Some(streams);
        self
    }

    /// Attach a TLS configuration.
    pub fn with_tls(mut self, cfg: IngressTlsConfig) -> Self {
        self.tls = Some(cfg);
        self
    }

    /// Attach an interceptor chain.
    pub fn with_interceptors(mut self, chain: GrpcIngressInterceptorChain) -> Self {
        self.interceptors = Some(chain);
        self
    }

    /// Set the compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = Some(mode);
        self
    }

    /// Allow unauthenticated callers.
    pub fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    /// Replace the default no-op audit sink.
    pub fn with_audit_sink(mut self, sink: Arc<dyn AuditSink>) -> Self {
        self.audit_sink = Some(sink);
        self
    }

    /// Enable gRPC reflection.
    pub fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }

    /// Consume the builder and produce a [`TonicGrpcServer`].
    pub fn build(self) -> TonicGrpcServer {
        let mut s = TonicGrpcServer::new(self.bind, self.handler);
        if let Some(v) = self.max_bytes {
            s = s.with_max_message_size(v);
        }
        if let Some(v) = self.max_concurrent_streams {
            s = s.with_max_concurrent_streams(v);
        }
        if let Some(v) = self.tls {
            s = s.with_tls(v);
        }
        if let Some(v) = self.interceptors {
            s = s.with_interceptors(v);
        }
        if let Some(v) = self.compression {
            s = s.with_compression(v);
        }
        s = s.allow_unauthenticated(self.allow_unauthenticated);
        if let Some(v) = self.audit_sink {
            s = s.with_audit_sink(v);
        }
        s = s.enable_reflection(self.enable_reflection);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc_ingress::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult};
    use crate::api::value::{GrpcMetadata, GrpcRequest, GrpcResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;

    struct TonicGrpcServerBuilderTestStub;
    impl GrpcIngress for TonicGrpcServerBuilderTestStub {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: vec![],
                    metadata: GrpcMetadata::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }

    fn stub() -> Arc<dyn GrpcIngress> {
        Arc::new(TonicGrpcServerBuilderTestStub)
    }

    #[test]
    fn test_new_creates_builder_with_bind_and_handler() {
        let _ = TonicGrpcServerBuilder::new("127.0.0.1:0", stub());
    }

    /// @covers: build
    #[test]
    fn test_build_produces_tonic_grpc_server() {
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .build();
        assert!(s.allow_unauthenticated);
    }

    /// @covers: with_max_message_size
    #[test]
    fn test_with_max_message_size_overrides_default() {
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .with_max_message_size(512)
            .build();
        assert_eq!(s.max_bytes, 512);
    }

    /// @covers: with_max_concurrent_streams
    #[test]
    fn test_with_max_concurrent_streams_sets_value() {
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .with_max_concurrent_streams(10)
            .build();
        assert_eq!(s.max_concurrent_streams, 10);
    }

    /// @covers: enable_reflection
    #[test]
    fn test_enable_reflection_sets_flag() {
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .enable_reflection()
            .build();
        assert!(s.is_reflection_enabled());
    }

    /// @covers: with_compression
    #[test]
    fn test_with_compression_sets_compression_mode() {
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .with_compression(CompressionMode::Gzip)
            .build();
        assert!(matches!(s.compression, CompressionMode::Gzip));
    }

    /// @covers: with_interceptors
    #[test]
    fn test_with_interceptors_assigns_chain() {
        let chain = GrpcIngressInterceptorChain::new();
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .with_interceptors(chain)
            .build();
        assert!(s.interceptors.is_empty());
    }

    /// @covers: with_audit_sink
    #[test]
    fn test_with_audit_sink_installs_custom_sink() {
        use crate::api::audit_sink::NoopAuditSink;
        let sink: Arc<dyn AuditSink> = Arc::new(NoopAuditSink);
        let _ = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .with_audit_sink(sink)
            .build();
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_attaches_tls_config_to_server() {
        use swe_edge_ingress_tls::IngressTlsConfig;
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .with_tls(IngressTlsConfig::tls("c.pem", "k.pem"))
            .build();
        assert!(s.tls.is_some());
    }

    /// @covers: allow_unauthenticated
    #[test]
    fn test_allow_unauthenticated_sets_flag_on_built_server() {
        let s = TonicGrpcServerBuilder::new("127.0.0.1:0", stub())
            .allow_unauthenticated()
            .build();
        assert!(s.allow_unauthenticated);
    }
}
