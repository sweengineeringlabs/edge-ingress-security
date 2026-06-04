//! gRPC request/response metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a gRPC request/response (headers).
///
/// Keys follow the gRPC metadata convention: lowercase ASCII with hyphens.
/// Binary values (suffix `-bin`) are base64-encoded by the transport;
/// use plain string values for text metadata.
///
/// Interceptors read and write metadata via the `headers` map.
/// Reserved `x-edge-*` keys are set by the transport layer — interceptors
/// MUST NOT overwrite them.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_transport::GrpcMetadata;
///
/// let mut meta = GrpcMetadata::default();
/// meta.headers.insert("x-request-id".to_string(), "abc-123".to_string());
///
/// assert_eq!(meta.headers.get("x-request-id").map(String::as_str), Some("abc-123"));
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrpcMetadata {
    /// Metadata key/value pairs (request headers or response trailers).
    pub headers: HashMap<String, String>,
}
