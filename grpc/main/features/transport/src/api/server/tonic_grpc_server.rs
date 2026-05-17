//! gRPC server declarations, error types, and builder methods.

use std::sync::Arc;

use swe_edge_ingress_tls::IngressTlsConfig;

use crate::api::audit_sink::{AuditSink, NoopAuditSink};
use crate::api::interceptor::GrpcInboundInterceptorChain;
use crate::api::port::grpc_inbound::GrpcInbound;
use crate::api::value_object::{CompressionMode, GrpcServerConfig};

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

/// Error returned by [`TonicGrpcServer::serve`].
#[derive(Debug, thiserror::Error)]
pub enum TonicServerError {
    /// Failed to bind the server socket.
    #[error("failed to bind to {0}: {1}")]
    Bind(String, #[source] std::io::Error),
    /// TLS acceptor construction failed.
    #[error("TLS: {0}")]
    Tls(#[source] swe_edge_ingress_tls::IngressTlsError),
    /// Server configuration was rejected.
    #[error("server config rejected: {0}")]
    Config(#[source] GrpcServerConfigError),
}

/// Error returned by [`TonicGrpcServer::from_config`].
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerConfigError {
    #[error(
        "tls_required is set but no TLS configuration supplied — \
         attach an IngressTlsConfig via with_tls(...) or call \
         allow_plaintext() to opt out"
    )]
    /// `tls_required` is set but no `IngressTlsConfig` was attached.
    TlsRequiredButMissing,
}

/// gRPC server that routes all unary requests through a [`GrpcInbound`] port.
pub struct TonicGrpcServer {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn GrpcInbound>,
    pub(crate) max_bytes: usize,
    pub(crate) max_concurrent_streams: u32,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) interceptors: GrpcInboundInterceptorChain,
    pub(crate) compression: CompressionMode,
    pub(crate) allow_unauthenticated: bool,
    pub(crate) audit_sink: Arc<dyn AuditSink>,
    pub(crate) enable_reflection: bool,
}

impl TonicGrpcServer {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn GrpcInbound>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            max_bytes: MAX_MESSAGE_BYTES,
            max_concurrent_streams: 100,
            tls: None,
            interceptors: GrpcInboundInterceptorChain::new(),
            compression: CompressionMode::None,
            allow_unauthenticated: false,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: false,
        }
    }

    /// Construct a server from a [`GrpcServerConfig`].
    pub fn from_config(
        config: &GrpcServerConfig,
        handler: Arc<dyn GrpcInbound>,
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
            interceptors: GrpcInboundInterceptorChain::new(),
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
    pub fn with_interceptors(mut self, chain: GrpcInboundInterceptorChain) -> Self {
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
    use crate::api::port::grpc_inbound::{GrpcHealthCheck, GrpcInbound, GrpcInboundResult};
    use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    struct DummyHandler;
    impl GrpcInbound for DummyHandler {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: vec![],
                    metadata: GrpcMetadata::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }

    /// @covers: TonicGrpcServer::new — creates server with defaults.
    #[test]
    fn test_new_creates_server_with_max_message_bytes_default() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler));
        assert_eq!(s.max_bytes, MAX_MESSAGE_BYTES);
        assert!(!s.allow_unauthenticated);
    }

    /// @covers: from_config — TlsRequiredButMissing when tls_required without tls config.
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

    /// @covers: allow_unauthenticated — sets flag.
    #[test]
    fn test_allow_unauthenticated_sets_the_flag() {
        let s =
            TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler)).allow_unauthenticated(true);
        assert!(s.allow_unauthenticated);
    }

    /// @covers: enable_reflection, is_reflection_enabled.
    #[test]
    fn test_enable_reflection_sets_and_reads_the_flag() {
        let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(DummyHandler)).enable_reflection(true);
        assert!(s.is_reflection_enabled());
    }

    /// @covers: GrpcServerConfigError — error message is descriptive.
    #[test]
    fn test_grpc_server_config_error_tls_required_has_descriptive_message() {
        let e = GrpcServerConfigError::TlsRequiredButMissing;
        assert!(e.to_string().contains("tls_required"));
    }
}
