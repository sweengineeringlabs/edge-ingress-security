//! gRPC status-code conversions and sanitization constant.

use crate::api::port::grpc_inbound::GrpcInboundError;
use crate::api::value_object::GrpcStatusCode;

/// Sanitized message returned to clients for any `Internal` server error.
/// The full server-side message is logged separately.
pub const SANITIZED_INTERNAL_MSG: &str = "internal server error";

/// Convert a [`tonic::Code`] into the crate-local [`GrpcStatusCode`].  Total.
pub fn from_tonic_code(code: tonic::Code) -> GrpcStatusCode {
    match code {
        tonic::Code::Ok => GrpcStatusCode::Ok,
        tonic::Code::Cancelled => GrpcStatusCode::Cancelled,
        tonic::Code::Unknown => GrpcStatusCode::Unknown,
        tonic::Code::InvalidArgument => GrpcStatusCode::InvalidArgument,
        tonic::Code::DeadlineExceeded => GrpcStatusCode::DeadlineExceeded,
        tonic::Code::NotFound => GrpcStatusCode::NotFound,
        tonic::Code::AlreadyExists => GrpcStatusCode::AlreadyExists,
        tonic::Code::PermissionDenied => GrpcStatusCode::PermissionDenied,
        tonic::Code::ResourceExhausted => GrpcStatusCode::ResourceExhausted,
        tonic::Code::FailedPrecondition => GrpcStatusCode::FailedPrecondition,
        tonic::Code::Aborted => GrpcStatusCode::Aborted,
        tonic::Code::OutOfRange => GrpcStatusCode::OutOfRange,
        tonic::Code::Unimplemented => GrpcStatusCode::Unimplemented,
        tonic::Code::Internal => GrpcStatusCode::Internal,
        tonic::Code::Unavailable => GrpcStatusCode::Unavailable,
        tonic::Code::DataLoss => GrpcStatusCode::DataLoss,
        tonic::Code::Unauthenticated => GrpcStatusCode::Unauthenticated,
    }
}

/// Convert a crate-local [`GrpcStatusCode`] into a [`tonic::Code`].  Total.
pub fn to_tonic_code(code: GrpcStatusCode) -> tonic::Code {
    match code {
        GrpcStatusCode::Ok => tonic::Code::Ok,
        GrpcStatusCode::Cancelled => tonic::Code::Cancelled,
        GrpcStatusCode::Unknown => tonic::Code::Unknown,
        GrpcStatusCode::InvalidArgument => tonic::Code::InvalidArgument,
        GrpcStatusCode::DeadlineExceeded => tonic::Code::DeadlineExceeded,
        GrpcStatusCode::NotFound => tonic::Code::NotFound,
        GrpcStatusCode::AlreadyExists => tonic::Code::AlreadyExists,
        GrpcStatusCode::PermissionDenied => tonic::Code::PermissionDenied,
        GrpcStatusCode::ResourceExhausted => tonic::Code::ResourceExhausted,
        GrpcStatusCode::FailedPrecondition => tonic::Code::FailedPrecondition,
        GrpcStatusCode::Aborted => tonic::Code::Aborted,
        GrpcStatusCode::OutOfRange => tonic::Code::OutOfRange,
        GrpcStatusCode::Unimplemented => tonic::Code::Unimplemented,
        GrpcStatusCode::Internal => tonic::Code::Internal,
        GrpcStatusCode::Unavailable => tonic::Code::Unavailable,
        GrpcStatusCode::DataLoss => tonic::Code::DataLoss,
        GrpcStatusCode::Unauthenticated => tonic::Code::Unauthenticated,
    }
}

/// Encode a [`GrpcStatusCode`] as the numeric `grpc-status` wire value.
pub fn to_wire(code: GrpcStatusCode) -> i32 {
    to_tonic_code(code) as i32
}

/// Parse a numeric `grpc-status` wire value into a [`GrpcStatusCode`].
/// Returns `Unknown` for unrecognized values per the gRPC spec.
pub fn from_wire(value: i32) -> GrpcStatusCode {
    from_tonic_code(tonic::Code::from(value))
}

/// Map a [`GrpcInboundError`] to `(tonic::Code, on-wire message)`.
///
/// `Internal(raw)` sanitizes the message for the wire and logs at WARN.
pub fn map_inbound_error(e: GrpcInboundError) -> (tonic::Code, String) {
    match e {
        GrpcInboundError::Status(code, msg) => (to_tonic_code(code), msg),
        GrpcInboundError::Internal(msg) => {
            tracing::warn!(server_internal_msg = %msg, "gRPC handler returned Internal — sanitizing for wire");
            (tonic::Code::Internal, SANITIZED_INTERNAL_MSG.to_owned())
        }
        GrpcInboundError::NotFound(m) => (tonic::Code::NotFound, m),
        GrpcInboundError::InvalidArgument(m) => (tonic::Code::InvalidArgument, m),
        GrpcInboundError::Unavailable(m) => (tonic::Code::Unavailable, m),
        GrpcInboundError::DeadlineExceeded(m) => (tonic::Code::DeadlineExceeded, m),
        GrpcInboundError::PermissionDenied(m) => (tonic::Code::PermissionDenied, m),
        GrpcInboundError::Unimplemented(m) => (tonic::Code::Unimplemented, m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_17: [GrpcStatusCode; 17] = [
        GrpcStatusCode::Ok,
        GrpcStatusCode::Cancelled,
        GrpcStatusCode::Unknown,
        GrpcStatusCode::InvalidArgument,
        GrpcStatusCode::DeadlineExceeded,
        GrpcStatusCode::NotFound,
        GrpcStatusCode::AlreadyExists,
        GrpcStatusCode::PermissionDenied,
        GrpcStatusCode::ResourceExhausted,
        GrpcStatusCode::FailedPrecondition,
        GrpcStatusCode::Aborted,
        GrpcStatusCode::OutOfRange,
        GrpcStatusCode::Unimplemented,
        GrpcStatusCode::Internal,
        GrpcStatusCode::Unavailable,
        GrpcStatusCode::DataLoss,
        GrpcStatusCode::Unauthenticated,
    ];

    /// @covers: from_tonic_code, to_tonic_code — round-trip for all 17 variants.
    #[test]
    fn test_round_trip_through_tonic_code_preserves_all_17_variants() {
        for code in ALL_17 {
            assert_eq!(from_tonic_code(to_tonic_code(code)), code);
        }
    }

    /// @covers: to_wire, from_wire — wire-value round-trip for all 17 variants.
    #[test]
    fn test_round_trip_through_wire_value_preserves_all_17_variants() {
        for code in ALL_17 {
            assert_eq!(from_wire(to_wire(code)), code);
        }
    }

    /// @covers: map_inbound_error — Internal sanitises the message.
    #[test]
    fn test_map_inbound_error_internal_returns_sanitized_message() {
        let (code, msg) = map_inbound_error(GrpcInboundError::Internal("secret/path".into()));
        assert_eq!(code, tonic::Code::Internal);
        assert_eq!(msg, SANITIZED_INTERNAL_MSG);
    }
}
