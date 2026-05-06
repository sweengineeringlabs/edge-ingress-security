//! SAF layer — HTTP inbound public facade.

pub use crate::api::handler_adapter::{HttpDecodeFn, HttpEncodeFn, HttpHandlerAdapter};
pub use crate::api::port::http_inbound::{HttpHealthCheck, HttpInbound, HttpInboundError, HttpInboundResult};
pub use crate::api::value_object::{FormPart, HttpAuth, HttpBody, HttpConfig, HttpMethod, HttpRequest, HttpResponse};
pub use crate::core::handler_dispatch::{HttpDispatcherError, HttpHandlerRegistryDispatcher};
pub use crate::core::server::{AxumHttpServer, AxumServerError, MAX_BODY_BYTES};
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};
