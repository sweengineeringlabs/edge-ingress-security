//! gRPC inbound port.

pub(crate) mod grpc_ingress;

pub use grpc_ingress::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
};
