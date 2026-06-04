//! gRPC timeout — `DEFAULT_DEADLINE` constant.

use std::time::Duration;

/// Server-side default deadline applied when the client did not send a
/// `grpc-timeout` header.
pub const DEFAULT_DEADLINE: Duration = Duration::from_secs(30);

pub use crate::api::types::transport::grpc_timeout_parser::GrpcTimeoutParser;
