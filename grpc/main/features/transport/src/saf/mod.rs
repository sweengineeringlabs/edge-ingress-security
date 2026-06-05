//! SAF layer — gRPC inbound public facade.

mod transport_svc;

pub use crate::api::types::application::ApplicationConfig;
pub use crate::api::types::audit::{AuditEvent, AuditEventBuilder, NoopAuditSink};
pub use crate::api::traits::{
    AuditSink, AuthorizationInterceptor, GrpcIngress, GrpcIngressInterceptor, GrpcServer,
};
pub use crate::api::types::grpc::{
    DecodeFn as GrpcDecodeFn, EncodeFn as GrpcEncodeFn, GrpcHandlerAdapter,
    GrpcHandlerRegistryDispatcher,
};
pub use crate::api::types::health::{
    HealthAggregate, HealthService, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD, WATCH_CHANNEL_CAPACITY,
};
pub use crate::api::types::ServingStatus;
pub use crate::api::types::interceptor::{
    GrpcIngressInterceptorChain, TraceContextInterceptor, EXTRACTED_TRACEPARENT,
    EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
pub use crate::api::error::{
    GrpcIngressError, GrpcServerConfigError, TonicServerError,
};
pub use crate::api::types::{GrpcHealthCheck, GrpcIngressResult, GrpcMessageStream};
pub use crate::api::types::server::{
    TonicGrpcServer, TonicGrpcServerBuilder, MAX_MESSAGE_BYTES,
    MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG,
};
pub use crate::api::types::internal::grpc_timeout_parser::{GrpcTimeoutParser, DEFAULT_DEADLINE};
pub use crate::api::types::internal::peer_identity_extractor::PeerIdentityExtractor;
pub use crate::api::types::internal::status_code_converter::{
    StatusCodeConverter, SANITIZED_INTERNAL_MSG,
};
pub use crate::api::value::{
    CompressionMode, GrpcMetadata, GrpcRequest, GrpcRequestBuilder, GrpcResponse, GrpcServerConfig,
    GrpcServerConfigBuilder, GrpcStatusCode, PeerIdentity, DEFAULT_MAX_CONCURRENT_STREAMS,
    DEFAULT_MAX_MESSAGE_BYTES, PEER_CERT_FINGERPRINT_SHA256, PEER_CN, PEER_IDENTITY, PEER_SAN_DNS,
    PEER_SAN_URI, RESERVED_PEER_PREFIXES,
};
pub use edge_domain::RequestContext;
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
pub use transport_svc::{create_config_builder, validate};
