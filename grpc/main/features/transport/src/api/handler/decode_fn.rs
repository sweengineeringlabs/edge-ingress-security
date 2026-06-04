//! Decode function pointer type for gRPC handler adapters.

use crate::api::port::grpc::GrpcIngressError;

/// Function pointer that decodes a typed request from raw protobuf bytes.
///
/// Implementations should return [`GrpcIngressError::InvalidArgument`]
/// when the bytes cannot be parsed — that surfaces as
/// `tonic::Code::InvalidArgument` on the wire.
pub type DecodeFn<Req> = fn(&[u8]) -> Result<Req, GrpcIngressError>;
