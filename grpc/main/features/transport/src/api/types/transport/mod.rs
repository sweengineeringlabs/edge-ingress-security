//! Transport layer public types — gRPC domain types, port, and interceptors.

pub mod application;
pub mod audit;
pub mod grpc;
pub mod grpc_timeout;
pub mod health;
pub mod interceptor;
pub mod port;
pub mod server;
pub mod status_codes;
pub mod value;

pub(crate) mod serving_status;

pub use application::ApplicationConfig;
pub use audit::{AuditEvent, AuditEventBuilder, NoopAuditSink};
pub use grpc::{GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher};
pub use grpc_timeout::GrpcTimeoutParser;
pub use health::{
    HealthAggregate, HealthService, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
    WATCH_CHANNEL_CAPACITY,
};
pub use interceptor::{
    GrpcIngressInterceptor, GrpcIngressInterceptorChain, TraceContextInterceptor,
    EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
pub use port::GrpcHealthCheck;
pub use server::{
    TonicGrpcServer, TonicGrpcServerBuilder, MAX_MESSAGE_BYTES,
    MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG,
};
pub use serving_status::ServingStatus;
pub use status_codes::StatusCodeConverter;
pub use value::{
    CompressionMode, GrpcMetadata, GrpcRequest, GrpcRequestBuilder, GrpcResponse, GrpcServerConfig,
    GrpcServerConfigBuilder, GrpcStatusCode, PeerIdentity, DEFAULT_MAX_CONCURRENT_STREAMS,
    DEFAULT_MAX_MESSAGE_BYTES, PEER_CERT_FINGERPRINT_SHA256, PEER_CN, PEER_IDENTITY, PEER_SAN_DNS,
    PEER_SAN_URI, RESERVED_PEER_PREFIXES,
};
