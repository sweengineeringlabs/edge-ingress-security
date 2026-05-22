//! `ReflectionRequest` — `ServerReflectionRequest.message_request` oneof.
//!
//! These mirror the proto3 messages from `grpc/reflection/v1alpha/reflection.proto`.
//! We do NOT depend on `prost` or `tonic-build`; the codecs in
//! [`crate::api::wire`] hand-encode exactly the field set we need.
//!
//! Keeping the variants exhaustively matched (no `_ => ` fallthroughs in
//! the codec) means that an unrecognised oneof variant is surfaced loudly at
//! compile time rather than swallowed at runtime.

/// `ServerReflectionRequest.message_request` oneof.
///
/// Variants we don't yet implement (`FileContainingExtension`,
/// `AllExtensionNumbersOfType`) are still represented so the dispatcher
/// can return a structured error response instead of panicking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReflectionRequest {
    /// `file_by_filename = 3` — request the FileDescriptorProto whose
    /// `name` is the given path (e.g. `"google/protobuf/empty.proto"`).
    FileByFilename(String),
    /// `file_containing_symbol = 4` — request the FileDescriptorProto
    /// that defines the fully-qualified symbol (e.g. `"pkg.MyService"`).
    FileContainingSymbol(String),
    /// `file_containing_extension = 5` — extension lookups are not yet
    /// implemented; surfacing the variant lets the dispatcher answer
    /// with `ErrorResponse(NOT_FOUND)` rather than dropping the request.
    FileContainingExtension {
        /// Containing message type, fully qualified.
        containing_type: String,
        /// Extension number.
        extension_number: i32,
    },
    /// `all_extension_numbers_of_type = 6` — also unimplemented.
    AllExtensionNumbersOfType(String),
    /// `list_services = 7` — return every registered service name.
    /// The string payload is ignored by reference servers; we mirror
    /// that and treat any value (including empty) as "list everything".
    ListServices(String),
    /// Request whose oneof was either absent or carried an unknown
    /// field number — we record it and answer with `INVALID_ARGUMENT`.
    Unknown,
}
