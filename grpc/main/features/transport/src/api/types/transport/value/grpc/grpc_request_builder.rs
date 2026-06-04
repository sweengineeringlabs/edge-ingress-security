//! Builder for GrpcRequest.

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use super::grpc_metadata::GrpcMetadata;
use super::grpc_request::GrpcRequest;

/// Fluent builder for [`GrpcRequest`].
pub struct GrpcRequestBuilder {
    method: String,
    body: Vec<u8>,
    deadline: Duration,
    metadata: Option<GrpcMetadata>,
    cancellation: Option<CancellationToken>,
}

impl GrpcRequestBuilder {
    /// Start building a request for the given gRPC method path.
    pub fn new(method: impl Into<String>, deadline: Duration) -> Self {
        Self {
            method: method.into(),
            body: Vec::new(),
            deadline,
            metadata: None,
            cancellation: None,
        }
    }

    /// Set the raw request body bytes.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    /// Set the request metadata (headers).
    pub fn metadata(mut self, metadata: GrpcMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Attach a cancellation token.
    pub fn cancellation(mut self, token: CancellationToken) -> Self {
        self.cancellation = Some(token);
        self
    }

    /// Build the [`GrpcRequest`].
    pub fn build(self) -> GrpcRequest {
        let mut req = GrpcRequest::new(self.method, self.body, self.deadline);
        if let Some(m) = self.metadata {
            req = req.with_metadata(m);
        }
        if let Some(t) = self.cancellation {
            req = req.with_cancellation(t);
        }
        req
    }
}
