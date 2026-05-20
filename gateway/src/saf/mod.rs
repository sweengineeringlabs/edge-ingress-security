//! SAF layer — inbound public facade.

mod builder;

pub use crate::api::application_config_builder::{build_file_input, ApplicationConfigBuilder};
pub use crate::api::architecture_config_builder::ArchitectureConfigBuilder;
pub use crate::api::daemon::{DaemonContext, DaemonRunner};
pub use crate::api::health_check::{HealthCheck, HealthStatus};
pub use crate::api::inbound_source::InboundSource;
pub use crate::api::traits::Validator;
pub use crate::api::ingress_error::{
    IngressError, IngressErrorCode, IngressResult, ResultIngressExt,
};
pub use crate::api::metrics::{FieldExtractor, MetricFields, MetricsCollector};
pub use crate::api::pagination::{PaginatedResponse, Pagination};
pub use crate::api::rate_limiter::{RateLimiter, RateLimiterBuilder, RateLimiterSpec};
pub use builder::{file_input, passthrough_validator};

// Domain crate re-exports
pub use edge_domain::{Handler, HandlerError, HandlerRegistry};
pub use swe_edge_ingress_grpc::{
    AuthorizationInterceptor, DecodeFn as GrpcDecodeFn, EncodeFn as GrpcEncodeFn,
    GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher, GrpcHealthCheck, GrpcInbound,
    GrpcInboundError, GrpcInboundInterceptor, GrpcInboundInterceptorChain, GrpcInboundResult,
    GrpcMessageStream, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode, TonicGrpcServer,
    TonicServerError, MAX_MESSAGE_BYTES,
};
pub use swe_edge_ingress_http::{
    AxumHttpServer, AxumServerError, FormPart, HttpAuth, HttpBody, HttpConfig, HttpDecodeFn,
    HttpEncodeFn, HttpHandlerAdapter, HttpHandlerRegistryDispatcher, HttpHealthCheck, HttpInbound,
    HttpInboundError, HttpInboundResult, HttpMethod, HttpRequest, HttpResponse, HttpStreamInbound,
    RequestContext, SseEvent, SseStream, WsChannel, WsMessage, WsReceiver, WsSender, MAX_BODY_BYTES,
};
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
