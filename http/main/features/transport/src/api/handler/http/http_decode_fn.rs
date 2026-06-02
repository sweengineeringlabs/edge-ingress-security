//! Decode function type alias for the HTTP adapter.

use crate::api::port::http_ingress_error::HttpIngressError;
use crate::api::value::HttpRequest;

/// Decodes a typed request from an inbound [`HttpRequest`].
pub type HttpDecodeFn<Req> = fn(&HttpRequest) -> Result<Req, HttpIngressError>;
