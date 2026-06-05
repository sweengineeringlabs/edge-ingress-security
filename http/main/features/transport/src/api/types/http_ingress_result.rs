//! Result type alias for HTTP inbound operations.

use crate::api::error::HttpIngressError;

/// Result type for HTTP inbound operations.
pub type HttpIngressResult<T> = Result<T, HttpIngressError>;
