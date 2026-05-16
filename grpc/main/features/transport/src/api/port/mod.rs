//! gRPC inbound port.
pub(crate) mod grpc_inbound;

#[allow(unused_imports)]
pub use grpc_inbound::{GrpcHealthCheck, GrpcInbound, GrpcInboundError, GrpcInboundResult};
