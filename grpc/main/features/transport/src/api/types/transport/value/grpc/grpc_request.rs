//! gRPC request envelope.

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use super::grpc_metadata::GrpcMetadata;

/// A gRPC request envelope handed to a [`crate::api::traits::GrpcIngress`] handler.
///
/// ## Mandatory deadline
///
/// Every inbound request carries a per-call deadline.  The server populates
/// it from the wire `grpc-timeout` header (or applies a server default when
/// the client did not send one).  Construction without a `Duration` is a
/// compile error — see [`GrpcRequest::new`].
///
/// ## Cancellation propagation
///
/// `cancellation` is a [`CancellationToken`] the server fires when the
/// client TCP/HTTP-2 stream is closed before the handler completes.
/// Long-running handlers SHOULD select on this token to abort early
/// instead of running to completion against a disconnected caller.
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use swe_edge_ingress_grpc_transport::GrpcRequest;
///
/// let req = GrpcRequest::new(
///     "/pkg.GreetService/Greet",
///     b"\x0a\x05Alice".to_vec(),  // protobuf-encoded payload
///     Duration::from_secs(5),
/// );
///
/// assert_eq!(req.method, "/pkg.GreetService/Greet");
/// assert_eq!(req.deadline, Duration::from_secs(5));
/// assert!(req.cancellation.is_none());
/// assert!(req.metadata.headers.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct GrpcRequest {
    /// Fully-qualified gRPC method path (e.g. `"/pkg.Service/Method"`).
    pub method: String,
    /// Raw protobuf-encoded request bytes.
    pub body: Vec<u8>,
    /// Request metadata (headers / trailers).
    pub metadata: GrpcMetadata,
    /// Per-call deadline (parsed from `grpc-timeout` or server default).
    pub deadline: Duration,
    /// Token fired when the underlying client stream is cancelled.
    pub cancellation: Option<CancellationToken>,
}

impl GrpcRequest {
    /// Create a new request with a mandatory per-call deadline.
    ///
    /// `deadline` is a required positional argument — there is no overload
    /// without it and no default.  Compile error if omitted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use swe_edge_ingress_grpc_transport::GrpcRequest;
    ///
    /// let req = GrpcRequest::new("/svc.Foo/Bar", vec![], Duration::from_millis(500));
    /// assert_eq!(req.deadline, Duration::from_millis(500));
    /// ```
    pub fn new(method: impl Into<String>, body: Vec<u8>, deadline: Duration) -> Self {
        Self {
            method: method.into(),
            body,
            metadata: GrpcMetadata::default(),
            deadline,
            cancellation: None,
        }
    }

    /// Replace the metadata block.
    pub fn with_metadata(mut self, metadata: GrpcMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Attach a cancellation token sourced from the surrounding HTTP/2 stream.
    pub fn with_cancellation(mut self, token: CancellationToken) -> Self {
        self.cancellation = Some(token);
        self
    }
}
