//! SAF layer — HTTP inbound public facade.
pub(crate) mod transport_svc;
pub use crate::api::error::{AxumServerError, HttpDispatcherError, HttpIngressError};
pub use crate::api::traits::http::{HttpDecodeFn, HttpEncodeFn};
pub use crate::api::types::handler::{HttpHandlerAdapter, HttpHandlerRegistryDispatcher};
pub use crate::api::types::server::{AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, MAX_BODY_BYTES};
pub use crate::api::types::HttpHealthCheck;
pub use crate::api::types::HttpIngressResult;
pub use crate::api::types::TransportSvc;
pub use crate::api::types::validator::HttpConfigValidator;
pub use crate::api::vo::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, SseEvent, SseStream, WsChannel, WsMessage, WsReceiver,
    WsSender,
};
pub use crate::api::HttpIngress;
pub use crate::api::HttpStream;
pub use edge_domain::RequestContext;
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
