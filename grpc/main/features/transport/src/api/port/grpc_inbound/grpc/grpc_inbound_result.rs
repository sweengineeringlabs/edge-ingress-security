//! Result type alias for gRPC inbound operations.

use super::grpc_inbound_error::GrpcInboundError;

/// Result type for gRPC inbound operations.
pub type GrpcInboundResult<T> = Result<T, GrpcInboundError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc_inbound::GrpcInboundError;

    #[test]
    fn test_grpc_inbound_result_is_result_alias() {
        let ok: GrpcInboundResult<u32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);
        let err: GrpcInboundResult<u32> = Err(GrpcInboundError::Internal("fail".into()));
        assert!(err.is_err());
    }
}
