//! gRPC inbound port.
pub(crate) mod grpc_inbound;

pub use grpc_inbound::{GrpcHealthCheck, GrpcInbound, GrpcInboundError, GrpcInboundResult};
