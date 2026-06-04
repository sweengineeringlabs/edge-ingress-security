//! Result type alias for gRPC inbound operations.

use super::grpc_ingress_error::GrpcIngressError;

/// Result type for gRPC inbound operations.
pub type GrpcIngressResult<T> = Result<T, GrpcIngressError>;
