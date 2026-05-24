//! Result type alias for gRPC inbound operations.

use super::grpc_ingress_error::GrpcIngressError;

/// Result type for gRPC inbound operations.
pub type GrpcIngressResult<T> = Result<T, GrpcIngressError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc::GrpcIngressError;

    #[test]
    fn test_grpc_ingress_result_is_result_alias() {
        let ok: GrpcIngressResult<u32> = Ok(42);
        assert!(matches!(ok, Ok(42)));
        let err: GrpcIngressResult<u32> = Err(GrpcIngressError::Internal("fail".into()));
        assert!(err.is_err());
    }
}
