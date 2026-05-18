//! Tonic gRPC server interface declaration.

/// Re-export of the primary tonic server type.
///
/// This file exists to satisfy the SEA rule requiring every core implementation
/// file to have a corresponding interface counterpart in the api/ layer.
#[allow(unused_imports)]
pub use crate::api::server::tonic::tonic_grpc_server::TonicGrpcServer;
