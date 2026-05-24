//! Integration tests for GrpcRequest.

use std::collections::HashMap;
use std::time::Duration;
use swe_edge_ingress_grpc_transport::{GrpcMetadata, GrpcRequest};
use tokio_util::sync::CancellationToken;

/// @covers: GrpcRequest::new
#[test]
fn test_new_stores_method_body_and_deadline() {
    let req = GrpcRequest::new("svc/Method", vec![1, 2], Duration::from_millis(250));
    assert_eq!(req.method, "svc/Method");
    assert_eq!(req.body, vec![1, 2]);
    assert_eq!(req.deadline, Duration::from_millis(250));
    assert!(req.cancellation.is_none());
}

/// @covers: GrpcRequest::with_cancellation
#[test]
fn test_with_cancellation_propagates_cancel_signal() {
    let token = CancellationToken::new();
    let req =
        GrpcRequest::new("svc/M", vec![], Duration::from_secs(1)).with_cancellation(token.clone());
    let stored = req.cancellation.as_ref().expect("token should be stored");
    assert!(!stored.is_cancelled());
    token.cancel();
    assert!(stored.is_cancelled());
}

/// @covers: GrpcRequest::with_metadata
#[test]
fn test_with_metadata_replaces_default_metadata() {
    let mut headers = HashMap::new();
    headers.insert("x-foo".to_string(), "bar".to_string());
    let meta = GrpcMetadata { headers };
    let req =
        GrpcRequest::new("svc/Method", vec![], Duration::from_secs(1)).with_metadata(meta.clone());
    assert_eq!(
        req.metadata.headers.get("x-foo").map(|s| s.as_str()),
        Some("bar")
    );
}

/// @covers: GrpcRequest
#[test]
fn test_grpc_request_holds_method_and_body() {
    let req = GrpcRequest {
        method: "svc/Method".into(),
        body: vec![1, 2],
        metadata: GrpcMetadata::default(),
        deadline: Duration::from_secs(1),
        cancellation: None,
    };
    assert_eq!(req.method, "svc/Method");
    assert_eq!(req.body, vec![1, 2]);
}
