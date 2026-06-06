//! SAF layer — HTTP inbound public facade.
pub(crate) mod transport_svc;
pub use crate::api::error::HttpIngressError;
pub use crate::api::handler::error::HttpDispatcherError;
pub use crate::api::server::error::AxumServerError;
pub use crate::api::handler::traits::{HttpDecodeFn, HttpEncodeFn};
pub use crate::api::handler::types::{HttpHandlerAdapter, HttpHandlerRegistryDispatcher};
pub use crate::api::server::types::{AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, MAX_BODY_BYTES};
pub use crate::api::types::HttpHealthCheck;
pub use crate::api::types::HttpIngressResult;
pub use crate::api::types::TransportSvc;
pub use crate::api::validator::types::HttpConfigValidator;
pub use crate::api::vo::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, SseEvent, SseStream, WsChannel, WsMessage, WsReceiver,
    WsSender,
};
pub use crate::api::HttpIngress;
pub use crate::api::HttpStream;
pub use edge_domain::RequestContext;
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
