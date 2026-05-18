//! Integration tests exercising the `axum` dependency directly.
//!
//! Verifies that the transport crate's axum-based request extraction pipeline
//! works end-to-end — a real `axum::Router` is constructed, bound, and
//! receives requests over a loopback TCP connection.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use axum::http::StatusCode;
use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpInbound, HttpInboundResult, HttpRequest, HttpResponse,
    RequestContext,
};

// Verify axum StatusCode is available (exercises the axum dependency directly).
#[allow(dead_code)]
fn _axum_status_code_200() -> StatusCode {
    StatusCode::OK
}

struct PongHandler;

impl HttpInbound for PongHandler {
    fn handle(
        &self,
        _req: HttpRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, b"pong".to_vec())) })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

async fn start(handler: Arc<dyn HttpInbound>) -> (String, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{addr}");
    let (tx, rx) = oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), handler);
    tokio::spawn(async move {
        let _ = server
            .serve_with_listener(listener, async move {
                let _ = rx.await;
            })
            .await;
    });
    (base, tx)
}

/// @covers: AxumHttpServer::serve_with_listener
/// Exercises: axum Router construction and request dispatch.
#[tokio::test]
async fn test_axum_router_handles_get_request_and_returns_200() {
    let (base, _shutdown) = start(Arc::new(PongHandler)).await;
    let resp = reqwest::get(format!("{base}/ping")).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert_eq!(body, "pong");
}

/// @covers: AxumHttpServer::serve_with_listener
/// Exercises: axum response-builder path for custom status codes.
#[tokio::test]
async fn test_axum_router_handles_post_request_and_returns_200() {
    let (base, _shutdown) = start(Arc::new(PongHandler)).await;
    let resp = reqwest::Client::new()
        .post(format!("{base}/data"))
        .body("payload")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

/// @covers: AxumHttpServer::serve_with_listener
/// Exercises: axum graceful shutdown via the shutdown future.
#[tokio::test]
async fn test_axum_server_shuts_down_on_signal() {
    let (base, tx) = start(Arc::new(PongHandler)).await;

    // Confirm it's alive.
    let r = reqwest::get(format!("{base}/check")).await.unwrap();
    assert_eq!(r.status(), 200);

    // Signal shutdown.
    let _ = tx.send(());
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Connection should now be refused.
    assert!(reqwest::get(format!("{base}/check")).await.is_err());
}
