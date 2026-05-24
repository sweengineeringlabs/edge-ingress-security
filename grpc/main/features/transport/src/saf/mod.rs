//! SAF layer — gRPC inbound public facade.

mod edge_ingress_grpc_transport_svc;

pub use crate::api::application_config::ApplicationConfig;
pub use swe_edge_configbuilder::create_config_builder;
pub use crate::api::audit_sink::{AuditEvent, AuditEventBuilder, AuditSink, NoopAuditSink};
pub use crate::api::grpc_timeout::{parse_grpc_timeout, DEFAULT_DEADLINE};
pub use crate::api::handler::{
    DecodeFn as GrpcDecodeFn, EncodeFn as GrpcEncodeFn, GrpcHandlerAdapter,
    GrpcHandlerRegistryDispatcher,
};
pub use crate::api::health_service::{
    HealthAggregate, HealthService, ServingStatus, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
    WATCH_CHANNEL_CAPACITY,
};
pub use crate::api::interceptor::trace_context_interceptor::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
pub use crate::api::interceptor::{
    AuthorizationInterceptor, GrpcIngressInterceptor, GrpcIngressInterceptorChain,
};
pub use crate::api::peer_identity::extract_peer_identity;
pub use crate::api::port::grpc_ingress::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
};
pub use crate::api::server::{
    GrpcServer, GrpcServerConfigError, TonicGrpcServer, TonicGrpcServerBuilder, TonicServerError,
    MAX_MESSAGE_BYTES, MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG,
};
pub use crate::api::status_codes::{
    from_tonic_code, from_wire, map_inbound_error, to_tonic_code, to_wire, SANITIZED_INTERNAL_MSG,
};
pub use crate::api::value_object::{
    is_reserved_peer_key, CompressionMode, GrpcMetadata, GrpcRequest, GrpcRequestBuilder,
    GrpcResponse, GrpcServerConfig, GrpcServerConfigBuilder, GrpcStatusCode, PeerIdentity,
    DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES, PEER_CERT_FINGERPRINT_SHA256,
    PEER_CN, PEER_IDENTITY, PEER_SAN_DNS, PEER_SAN_URI, RESERVED_PEER_PREFIXES,
};
pub use edge_domain::RequestContext;
pub use edge_ingress_grpc_transport_svc::validate;
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
