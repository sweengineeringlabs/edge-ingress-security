//! gRPC inbound port types.

pub(crate) mod grpc;

pub use grpc::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundError, GrpcInboundResult, GrpcMessageStream,
};
