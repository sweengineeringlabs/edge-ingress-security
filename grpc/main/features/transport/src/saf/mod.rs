//! SAF layer — gRPC inbound public facade.

mod transport_svc;

pub use crate::api::application::ApplicationConfig;
pub use crate::api::audit::{AuditEvent, AuditEventBuilder, AuditSink, NoopAuditSink};
pub use crate::api::handler::{
    DecodeFn as GrpcDecodeFn, EncodeFn as GrpcEncodeFn, GrpcHandlerAdapter,
    GrpcHandlerRegistryDispatcher,
};
pub use crate::api::health::{
    HealthAggregate, HealthService, ServingStatus, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
    WATCH_CHANNEL_CAPACITY,
};
pub use crate::api::interceptor::{
    AuthorizationInterceptor, GrpcIngressInterceptor, GrpcIngressInterceptorChain,
};
pub use crate::api::interceptor::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
pub use crate::api::port::grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
};
pub use crate::api::server::{
    GrpcServer, GrpcServerConfigError, TonicGrpcServer, TonicGrpcServerBuilder, TonicServerError,
    MAX_MESSAGE_BYTES, MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG,
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
