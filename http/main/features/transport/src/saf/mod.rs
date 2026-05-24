//! SAF layer — HTTP inbound public facade.
pub(crate) mod edge_ingress_http_transport_svc;
pub use crate::api::handler::http::http_decode_fn::HttpDecodeFn;
pub use crate::api::handler::http::http_dispatcher_error::HttpDispatcherError;
pub use crate::api::handler::http::http_encode_fn::HttpEncodeFn;
pub use crate::api::handler::http::http_handler_adapter::HttpHandlerAdapter;
pub use crate::api::handler::http::http_handler_registry_dispatcher::HttpHandlerRegistryDispatcher;
pub use crate::api::port::http::HttpStream;
pub use crate::api::port::http_health_check::HttpHealthCheck;
pub use crate::api::port::http_ingress::HttpIngress;
pub use crate::api::port::http_ingress_error::HttpIngressError;
pub use crate::api::port::http_ingress_result::HttpIngressResult;
pub use crate::api::server::axum::axum_http_server_builder::AxumHttpServerBuilder;
pub use crate::api::server::axum::axum_server_error::AxumServerError;
pub use crate::api::types::server::axum_http_server::{AxumHttpServer, MAX_BODY_BYTES};
pub use crate::api::value_object::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, SseEvent, SseStream, WsChannel, WsMessage, WsReceiver,
    WsSender,
};
pub use edge_domain::RequestContext;
pub use edge_ingress_http_transport_svc::{create_config_builder, validate};
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
