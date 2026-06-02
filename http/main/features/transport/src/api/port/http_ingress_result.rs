//! Result type alias for HTTP inbound operations.

use crate::api::port::http_ingress_error::HttpIngressError;

/// Result type for HTTP inbound operations.
pub type HttpIngressResult<T> = Result<T, HttpIngressError>;
