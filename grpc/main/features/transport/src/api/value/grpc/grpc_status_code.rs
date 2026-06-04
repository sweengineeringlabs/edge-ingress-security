//! gRPC status codes (mirrors tonic/gRPC standard codes).

use serde::{Deserialize, Serialize};

/// A gRPC status code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrpcStatusCode {
    /// Request succeeded (code 0).
    Ok,
    /// Request was cancelled by the client.
    Cancelled,
    /// Unknown error.
    Unknown,
    /// Invalid argument supplied.
    InvalidArgument,
    /// Deadline expired before the operation completed.
    DeadlineExceeded,
    /// Resource not found.
    NotFound,
    /// Resource already exists.
    AlreadyExists,
    /// Caller lacks permission.
    PermissionDenied,
    /// Resource quota or rate limit exhausted.
    ResourceExhausted,
    /// Pre-condition for the operation not met.
    FailedPrecondition,
    /// Operation aborted.
    Aborted,
    /// Value or index out of valid range.
    OutOfRange,
    /// Operation not implemented.
    Unimplemented,
    /// Internal server error.
    Internal,
    /// Service temporarily unavailable.
    Unavailable,
    /// Unrecoverable data loss or corruption.
    DataLoss,
    /// Request lacks valid authentication credentials.
    Unauthenticated,
}

