//! Error type for gRPC inbound operations.

use crate::api::value::GrpcStatusCode;

/// Error type for gRPC inbound operations.
///
/// `Status(GrpcStatusCode, String)` is the canonical generic variant for any
/// gRPC error condition — handlers SHOULD prefer it over the named variants
/// when they need a status code that is not Internal/NotFound/etc.  Both
/// representations are recognised by [`crate::core::status_codes::map_inbound_error`].
///
/// The named variants (`Internal`, `NotFound`, `InvalidArgument`,
/// `Unavailable`, `DeadlineExceeded`, `PermissionDenied`, `Unimplemented`)
/// are kept for ergonomic call sites and for backwards source compatibility.
///
/// ## Hygiene contract
///
/// The string carried by `Internal(_)` is treated as a *server-side log
/// message* and may contain stack-traces or struct names.  The dispatch
/// layer logs it at WARN with `tracing::warn!`, then surfaces only a
/// fixed sanitized string on the wire.  Other variants pass their
/// message through verbatim — they are expected to be already
/// caller-safe (e.g. "no such row", "invalid argument 'foo'").
#[derive(Debug, thiserror::Error)]
pub enum GrpcIngressError {
    /// A gRPC status code with a sanitized message.  Preferred for new code.
    #[error("status {0:?}: {1}")]
    Status(GrpcStatusCode, String),
    /// Internal server error.  String is logged but NEVER sent on the wire.
    #[error("internal: {0}")]
    Internal(String),
    /// Resource not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// Request argument failed validation.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    /// Service unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// Request deadline exceeded.
    #[error("deadline exceeded: {0}")]
    DeadlineExceeded(String),
    /// Caller lacks permission.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// Method not implemented.
    #[error("unimplemented: {0}")]
    Unimplemented(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_ingress_error_internal_formats_correctly() {
        let err = GrpcIngressError::Internal("fail".into());
        assert!(err.to_string().contains("fail"));
    }

    #[test]
    fn test_grpc_ingress_error_status_variant_carries_code_and_message() {
        let err = GrpcIngressError::Status(GrpcStatusCode::Aborted, "tx aborted".into());
        let s = err.to_string();
        assert!(s.contains("Aborted"));
        assert!(s.contains("tx aborted"));
    }
}
