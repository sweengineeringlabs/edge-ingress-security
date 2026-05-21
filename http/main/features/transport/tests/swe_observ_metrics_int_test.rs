//! Integration tests exercising the `swe-observ-metrics` dependency directly.
//!
//! Verifies that the metrics instrumentation in `HttpHandlerRegistryDispatcher`
//! records the expected counters and histograms via the `MetricsProvider` trait.

use std::sync::Arc;

use edge_domain::{Handler, HandlerError, HandlerRegistry};
use futures::future::BoxFuture;
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

use swe_edge_ingress_http::{
    HttpHandlerAdapter, HttpHandlerRegistryDispatcher, HttpInbound, HttpInboundError, HttpRequest,
    HttpResponse, RequestContext,
};

// ── Ping handler ──────────────────────────────────────────────────────────────

struct PingHandler;

#[async_trait::async_trait]
impl Handler<HttpRequest, HttpResponse> for PingHandler {
    fn id(&self) -> &str {
        "ping"
    }
    fn pattern(&self) -> &str {
        "/ping"
    }
    async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
        Ok(HttpResponse::new(200, b"pong".to_vec()))
    }
}

// ── Error handler ─────────────────────────────────────────────────────────────

struct BoomHandler;

#[async_trait::async_trait]
impl Handler<HttpRequest, HttpResponse> for BoomHandler {
    fn id(&self) -> &str {
        "boom"
    }
    fn pattern(&self) -> &str {
        "/boom"
    }
    async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("boom".into()))
    }
}

fn identity_decode(req: &HttpRequest) -> Result<HttpRequest, HttpInboundError> {
    Ok(req.clone())
}

fn identity_encode(r: HttpResponse) -> HttpResponse {
    r
}

fn ctx() -> RequestContext {
    RequestContext::unauthenticated()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// @covers: HttpHandlerRegistryDispatcher::with_metrics
/// Exercises: swe_observ_metrics — edge_handler_requests_total incremented on success.
#[tokio::test]
async fn test_swe_observ_metrics_records_handler_requests_total_counter() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let dispatcher = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        .with_metrics(Arc::clone(&provider));
    dispatcher
        .register(HttpHandlerAdapter::new(
            Arc::new(PingHandler),
            identity_decode,
            identity_encode,
        ))
        .unwrap();

    dispatcher
        .handle(HttpRequest::get("/ping"), ctx())
        .await
        .unwrap();

    let snaps = provider.export();
    assert!(
        snaps
            .iter()
            .any(|s| s.name == "edge_handler_requests_total" && s.value == 1.0),
        "edge_handler_requests_total not found: {snaps:?}"
    );
}

/// @covers: HttpHandlerRegistryDispatcher::with_metrics
/// Exercises: swe_observ_metrics — edge_handler_errors_total incremented on failure.
#[tokio::test]
async fn test_swe_observ_metrics_records_handler_errors_total_on_handler_failure() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let dispatcher = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        .with_metrics(Arc::clone(&provider));
    dispatcher
        .register(HttpHandlerAdapter::new(
            Arc::new(BoomHandler),
            identity_decode,
            identity_encode,
        ))
        .unwrap();

    let _ = dispatcher.handle(HttpRequest::get("/boom"), ctx()).await;

    let snaps = provider.export();
    assert!(
        snaps
            .iter()
            .any(|s| s.name == "edge_handler_errors_total" && s.value == 1.0),
        "edge_handler_errors_total not found: {snaps:?}"
    );
}

/// @covers: HttpHandlerRegistryDispatcher::with_metrics
/// Exercises: swe_observ_metrics — edge_handler_latency_us histogram recorded.
#[tokio::test]
async fn test_swe_observ_metrics_records_handler_latency_histogram() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let dispatcher = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        .with_metrics(Arc::clone(&provider));
    dispatcher
        .register(HttpHandlerAdapter::new(
            Arc::new(PingHandler),
            identity_decode,
            identity_encode,
        ))
        .unwrap();

    dispatcher
        .handle(HttpRequest::get("/ping"), ctx())
        .await
        .unwrap();

    let snaps = provider.export();
    assert!(
        snaps.iter().any(|s| s.name == "edge_handler_latency_us"),
        "edge_handler_latency_us not found: {snaps:?}"
    );
}

/// @covers: HttpHandlerRegistryDispatcher::with_metrics
/// Exercises: swe_observ_metrics — multiple requests accumulate counter correctly.
#[tokio::test]
async fn test_swe_observ_metrics_accumulates_counter_across_multiple_requests() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let dispatcher = HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        .with_metrics(Arc::clone(&provider));
    dispatcher
        .register(HttpHandlerAdapter::new(
            Arc::new(PingHandler),
            identity_decode,
            identity_encode,
        ))
        .unwrap();

    for _ in 0..3 {
        dispatcher
            .handle(HttpRequest::get("/ping"), ctx())
            .await
            .unwrap();
    }

    let snaps = provider.export();
    let total = snaps
        .iter()
        .find(|s| s.name == "edge_handler_requests_total")
        .map(|s| s.value)
        .unwrap_or(0.0);
    assert_eq!(total, 3.0, "expected 3 requests, got {total}");
}
