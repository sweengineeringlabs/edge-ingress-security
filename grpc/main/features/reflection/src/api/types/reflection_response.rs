//! Subset of `ServerReflectionResponse.message_response` we generate.

/// Subset of `ServerReflectionResponse.message_response` we generate.
///
/// `ListServicesResponse` and `FileDescriptorResponse` are the two paths
/// grpcurl exercises by default; `ErrorResponse` covers everything else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReflectionResponse {
    /// Successful list-services response — one `ServiceResponse` per name.
    ListServices(Vec<String>),
    /// Successful file-descriptor response — one or more raw
    /// `FileDescriptorProto` byte buffers.
    FileDescriptor(Vec<Vec<u8>>),
    /// Structured error response — `error_code` is a gRPC status code.
    Error {
        /// gRPC status code (numeric).
        error_code: i32,
        /// Human-readable error message.
        error_message: String,
    },
}
