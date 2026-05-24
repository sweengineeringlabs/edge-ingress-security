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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_builder_with_method_and_deadline() {
        let b = GrpcRequestBuilder::new("/svc/M", Duration::from_secs(1));
        assert_eq!(b.method, "/svc/M");
    }

    /// @covers: build
    #[test]
    fn test_build_produces_grpc_request_with_all_fields() {
        let req = GrpcRequestBuilder::new("/svc/M", Duration::from_millis(500))
            .body(vec![1, 2, 3])
            .build();
        assert_eq!(req.method, "/svc/M");
        assert_eq!(req.body, vec![1, 2, 3]);
        assert_eq!(req.deadline, Duration::from_millis(500));
    }

    /// @covers: body
    #[test]
    fn test_body_sets_request_bytes() {
        let req = GrpcRequestBuilder::new("/svc/M", Duration::from_secs(1))
            .body(b"hello".to_vec())
            .build();
        assert_eq!(req.body, b"hello");
    }

    /// @covers: metadata
    #[test]
    fn test_metadata_replaces_default_metadata() {
        use std::collections::HashMap;
        let mut headers = HashMap::new();
        headers.insert("x-foo".to_string(), "bar".to_string());
        let meta = GrpcMetadata { headers };
        let req = GrpcRequestBuilder::new("/svc/M", Duration::from_secs(1))
            .metadata(meta)
            .build();
        assert_eq!(
            req.metadata.headers.get("x-foo").map(|s| s.as_str()),
            Some("bar")
        );
    }

    /// @covers: cancellation
    #[test]
    fn test_cancellation_attaches_token() {
        let token = CancellationToken::new();
        let req = GrpcRequestBuilder::new("/svc/M", Duration::from_secs(1))
            .cancellation(token.clone())
            .build();
        assert!(!req.cancellation.as_ref().unwrap().is_cancelled());
        token.cancel();
        assert!(req.cancellation.unwrap().is_cancelled());
    }
}
