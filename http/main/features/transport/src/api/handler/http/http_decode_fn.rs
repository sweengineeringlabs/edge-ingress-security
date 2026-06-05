//! Decode function type alias for the HTTP adapter.

use crate::api::error::HttpIngressError;
use crate::api::vo::HttpRequest;

/// Decodes a typed request from an inbound [`HttpRequest`].
pub type HttpDecodeFn<Req> = fn(&HttpRequest) -> Result<Req, HttpIngressError>;
