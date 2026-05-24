//! gRPC status-code conversions and sanitization constant.

use crate::api::port::grpc_ingress::GrpcIngressError;
use crate::api::value_object::GrpcStatusCode;

/// Sanitized message returned to clients for any `Internal` server error.
/// The full server-side message is logged separately.
pub const SANITIZED_INTERNAL_MSG: &str = "internal server error";

/// Converter for gRPC status codes between representations.
pub struct StatusCodeConverter;

impl StatusCodeConverter {
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
        Self::to_tonic_code(code) as i32
    }

    /// Parse a numeric `grpc-status` wire value into a [`GrpcStatusCode`].
    /// Returns `Unknown` for unrecognized values per the gRPC spec.
    pub fn from_wire(value: i32) -> GrpcStatusCode {
        Self::from_tonic_code(tonic::Code::from(value))
    }

    /// Map a [`GrpcIngressError`] to `(tonic::Code, on-wire message)`.
    ///
    /// `Internal(raw)` sanitizes the message for the wire and logs at WARN.
    pub fn map_inbound_error(e: GrpcIngressError) -> (tonic::Code, String) {
        match e {
            GrpcIngressError::Status(code, msg) => (Self::to_tonic_code(code), msg),
            GrpcIngressError::Internal(msg) => {
                tracing::warn!(server_internal_msg = %msg, "gRPC handler returned Internal — sanitizing for wire");
                (tonic::Code::Internal, SANITIZED_INTERNAL_MSG.to_owned())
            }
            GrpcIngressError::NotFound(m) => (tonic::Code::NotFound, m),
            GrpcIngressError::InvalidArgument(m) => (tonic::Code::InvalidArgument, m),
            GrpcIngressError::Unavailable(m) => (tonic::Code::Unavailable, m),
            GrpcIngressError::DeadlineExceeded(m) => (tonic::Code::DeadlineExceeded, m),
            GrpcIngressError::PermissionDenied(m) => (tonic::Code::PermissionDenied, m),
            GrpcIngressError::Unimplemented(m) => (tonic::Code::Unimplemented, m),
        }
    }
}

/// Backward-compatibility wrapper for from_tonic_code.
pub fn from_tonic_code(code: tonic::Code) -> GrpcStatusCode {
    StatusCodeConverter::from_tonic_code(code)
}

/// Backward-compatibility wrapper for to_tonic_code.
pub fn to_tonic_code(code: GrpcStatusCode) -> tonic::Code {
    StatusCodeConverter::to_tonic_code(code)
}

/// Backward-compatibility wrapper for to_wire.
pub fn to_wire(code: GrpcStatusCode) -> i32 {
    StatusCodeConverter::to_wire(code)
}

/// Backward-compatibility wrapper for from_wire.
pub fn from_wire(value: i32) -> GrpcStatusCode {
    StatusCodeConverter::from_wire(value)
}

/// Backward-compatibility wrapper for map_inbound_error.
pub fn map_inbound_error(e: GrpcIngressError) -> (tonic::Code, String) {
    StatusCodeConverter::map_inbound_error(e)
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

    /// @covers: StatusCodeConverter::from_tonic_code
    #[test]
    fn test_from_tonic_code_round_trips_all_17_variants() {
        for code in ALL_17 {
            let tonic = StatusCodeConverter::to_tonic_code(code);
            assert_eq!(StatusCodeConverter::from_tonic_code(tonic), code);
        }
    }

    /// @covers: StatusCodeConverter::to_tonic_code
    #[test]
    fn test_to_tonic_code_round_trips_all_17_variants() {
        for code in ALL_17 {
            let tonic = StatusCodeConverter::to_tonic_code(code);
            assert_eq!(StatusCodeConverter::from_tonic_code(tonic), code);
        }
    }

    /// @covers: StatusCodeConverter::to_wire
    #[test]
    fn test_to_wire_round_trips_all_17_variants() {
        for code in ALL_17 {
            assert_eq!(
                StatusCodeConverter::from_wire(StatusCodeConverter::to_wire(code)),
                code
            );
        }
    }

    /// @covers: StatusCodeConverter::from_wire
    #[test]
    fn test_from_wire_round_trips_all_17_variants() {
        for code in ALL_17 {
            assert_eq!(
                StatusCodeConverter::from_wire(StatusCodeConverter::to_wire(code)),
                code
            );
        }
    }

    /// @covers: StatusCodeConverter::map_inbound_error
    #[test]
    fn test_map_inbound_error_internal_returns_sanitized_message() {
        let (code, msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::Internal(
            "secret/path".into(),
        ));
        assert_eq!(code, tonic::Code::Internal);
        assert_eq!(msg, SANITIZED_INTERNAL_MSG);
    }
}
