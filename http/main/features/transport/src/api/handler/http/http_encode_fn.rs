//! Encode function type alias for the HTTP adapter.

use crate::api::value::HttpResponse;

/// Encodes a typed response into an [`HttpResponse`].
pub type HttpEncodeFn<Resp> = fn(Resp) -> HttpResponse;
