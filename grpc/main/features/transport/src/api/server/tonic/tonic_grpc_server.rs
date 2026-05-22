//! gRPC server — TonicGrpcServer declarations and builder methods.

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;

use crate::api::audit_sink::{AuditSink, NoopAuditSink};
use crate::api::interceptor::GrpcIngressInterceptorChain;
use crate::api::port::grpc_ingress::GrpcIngress;
use crate::api::value_object::{CompressionMode, GrpcServerConfig};

use crate::api::server::grpc::grpc_server_config_error::GrpcServerConfigError;

/// Panic message when no `AuthorizationInterceptor` is registered and `allow_unauthenticated` is false.
pub const MISSING_AUTHORIZATION_INTERCEPTOR_MSG: &str =
    "gRPC server requires an AuthorizationInterceptor in the chain \
     (e.g. swe-edge-ingress-grpc-authz::AuthzInterceptor). To explicitly run \
     without authz, set `allow_unauthenticated = true` in \
     GrpcServerConfig (logged at startup as a warning).";

/// Warning logged at startup when gRPC reflection is enabled.
pub const REFLECTION_ENABLED_WARN_MSG: &str =
    "gRPC reflection enabled — exposes service surface to anyone reaching this endpoint. \
     Disable in production deployments.";

/// Default maximum inbound message size (4 MiB).
pub const MAX_MESSAGE_BYTES: usize = 4 * 1_024 * 1_024; // 4 MiB

/// gRPC server that routes all unary requests through a [`GrpcIngress`] port.
pub struct TonicGrpcServer {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn GrpcIngress>,
    pub(crate) max_bytes: usize,
    pub(crate) max_concurrent_streams: u32,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) interceptors: GrpcIngressInterceptorChain,
    pub(crate) compression: CompressionMode,
    pub(crate) allow_unauthenticated: bool,
    pub(crate) audit_sink: Arc<dyn AuditSink>,
    pub(crate) enable_reflection: bool,
}

impl TonicGrpcServer {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            max_bytes: MAX_MESSAGE_BYTES,
            max_concurrent_streams: 100,
            tls: None,
            interceptors: GrpcIngressInterceptorChain::new(),
            compression: CompressionMode::None,
            allow_unauthenticated: false,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: false,
        }
    }

    /// Construct a server from a [`GrpcServerConfig`].
    pub fn from_config(
        config: &GrpcServerConfig,
        handler: Arc<dyn GrpcIngress>,
    ) -> Result<Self, GrpcServerConfigError> {
        if config.tls_required && config.tls.is_none() {
            return Err(GrpcServerConfigError::TlsRequiredButMissing);
        }
        Ok(Self {
            bind: config.bind.to_string(),
            handler,
            max_bytes: config.max_message_bytes,
            max_concurrent_streams: config.max_concurrent_streams,
            tls: config.tls.clone(),
            interceptors: GrpcIngressInterceptorChain::new(),
            compression: config.compression,
            allow_unauthenticated: config.allow_unauthenticated,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: config.enable_reflection,
        })
    }

    /// Enable or disable gRPC server reflection.
    pub fn enable_reflection(mut self, enable: bool) -> Self {
        self.enable_reflection = enable;
        self
    }

    /// Returns `true` if gRPC reflection is enabled.
    pub fn is_reflection_enabled(&self) -> bool {
        self.enable_reflection
    }

    /// Replace the default no-op audit sink with a custom implementation.
    pub fn with_audit_sink(mut self, sink: Arc<dyn AuditSink>) -> Self {
        self.audit_sink = sink;
        self
    }

    /// Allow requests without an `AuthorizationInterceptor` in the chain.
    pub fn allow_unauthenticated(mut self, allow: bool) -> Self {
        self.allow_unauthenticated = allow;
        self
    }

    /// Override the maximum inbound message size in bytes.
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_bytes = size;
        self
    }

    /// Override the maximum number of concurrent HTTP/2 streams.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    /// Attach an interceptor chain that runs before and after each dispatch.
    pub fn with_interceptors(mut self, chain: GrpcIngressInterceptorChain) -> Self {
        self.interceptors = chain;
        self
    }

    /// Set the response compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Attach a TLS configuration (enables mTLS when a CA cert is provided).
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc_ingress::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult};
    use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    struct DummyHandler;
    impl GrpcIngress for DummyHandler {
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

    #[test]
    fn test_new_creates_server_with_max_message_bytes_default() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler));
        assert_eq!(s.max_bytes, MAX_MESSAGE_BYTES);
        assert!(!s.allow_unauthenticated);
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_returns_err_when_tls_required_but_no_tls_config() {
        use crate::api::value_object::GrpcServerConfig;
        let cfg = GrpcServerConfig {
            tls_required: true,
            ..Default::default()
        };
        let res = TonicGrpcServer::from_config(&cfg, Arc::new(DummyHandler));
        assert!(res.is_err());
    }

    /// @covers: allow_unauthenticated
    #[test]
    fn test_allow_unauthenticated_sets_the_flag() {
        let s =
            TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler)).allow_unauthenticated(true);
        assert!(s.allow_unauthenticated);
    }

    /// @covers: enable_reflection
    #[test]
    fn test_enable_reflection_sets_the_flag() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler)).enable_reflection(true);
        assert!(s.is_reflection_enabled());
    }

    /// @covers: is_reflection_enabled
    #[test]
    fn test_is_reflection_enabled_false_by_default() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler));
        assert!(!s.is_reflection_enabled());
    }

    /// @covers: with_audit_sink
    #[test]
    fn test_with_audit_sink_installs_provided_sink() {
        use crate::api::audit_sink::AuditEvent;
        use crate::api::value_object::GrpcStatusCode;
        use std::sync::Mutex;
        use std::time::SystemTime;
        struct CountingSink(Arc<Mutex<usize>>);
        impl AuditSink for CountingSink {
            fn record(&self, _: AuditEvent) {
                *self.0.lock().unwrap() += 1;
            }
        }
        let calls = Arc::new(Mutex::new(0usize));
        let sink: Arc<dyn AuditSink> = Arc::new(CountingSink(calls.clone()));
        let server =
            TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler)).with_audit_sink(sink);
        server.audit_sink.record(AuditEvent {
            timestamp: SystemTime::UNIX_EPOCH,
            method: "/x".into(),
            identity: None,
            status: GrpcStatusCode::Ok,
            duration_ms: 0,
        });
        assert_eq!(*calls.lock().unwrap(), 1);
    }

    /// @covers: with_max_message_size
    #[test]
    fn test_with_max_message_size_overrides_default() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .allow_unauthenticated(true)
            .with_max_message_size(1024);
        assert_eq!(s.max_bytes, 1024);
    }

    /// @covers: with_max_concurrent_streams
    #[test]
    fn test_with_max_concurrent_streams_sets_value() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .allow_unauthenticated(true)
            .with_max_concurrent_streams(32);
        assert_eq!(s.max_concurrent_streams, 32);
    }

    /// @covers: with_interceptors
    #[test]
    fn test_with_interceptors_assigns_chain() {
        use crate::api::interceptor::GrpcIngressInterceptorChain;
        let chain = GrpcIngressInterceptorChain::new();
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .allow_unauthenticated(true)
            .with_interceptors(chain);
        drop(s);
    }

    /// @covers: with_compression
    #[test]
    fn test_with_compression_stores_mode() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .allow_unauthenticated(true)
            .with_compression(CompressionMode::Gzip);
        assert!(matches!(s.compression, CompressionMode::Gzip));
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_sets_config() {
        let cfg = swe_edge_ingress_tls::IngressTlsConfig::tls("cert.pem", "key.pem");
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler))
            .allow_unauthenticated(true)
            .with_tls(cfg);
        assert!(s.tls.is_some());
    }
}
