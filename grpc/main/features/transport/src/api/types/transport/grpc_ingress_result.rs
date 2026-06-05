//! Result type alias for gRPC inbound operations.

use crate::api::error::GrpcIngressError;

/// Result type for gRPC inbound operations.
pub type GrpcIngressResult<T> = Result<T, GrpcIngressError>;
