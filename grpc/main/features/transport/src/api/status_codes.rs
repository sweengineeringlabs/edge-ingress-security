//! gRPC status-code conversions and sanitization constant.

/// Sanitized message returned to clients for any `Internal` server error.
/// The full server-side message is logged separately.
pub const SANITIZED_INTERNAL_MSG: &str = "internal server error";

pub use crate::api::types::StatusCodeConverter;
