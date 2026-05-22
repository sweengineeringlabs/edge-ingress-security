//! gRPC inbound port types.

pub(crate) mod grpc;

pub use grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
};
