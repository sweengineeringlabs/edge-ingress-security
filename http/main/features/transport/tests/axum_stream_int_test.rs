//! Integration tests — SSE push and WebSocket dispatch via AxumHttpServer.
// @allow: no_mocks_in_integration — stub impls required to exercise the streaming API surface

use std::sync::Arc;

use futures::future::BoxFuture;
use futures::stream;
use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    HttpStream, RequestContext, SseEvent, SseStream, WsChannel,
};
use tokio::net::TcpListener;

// ── Stub non-streaming handler ────────────────────────────────────────────────

struct AxumStreamStubHttpIngress;
impl HttpIngress for AxumStreamStubHttpIngress {
    fn handle(
        &self,
        _: HttpRequest,
        _: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, b"ok".to_vec())) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

// ── SSE handler stub ──────────────────────────────────────────────────────────

struct AxumStreamStubSseHandler;
impl HttpStream for AxumStreamStubSseHandler {
    fn handle_sse(
        &self,
        _: HttpRequest,
        _: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
        Box::pin(async {
            let events: SseStream = Box::pin(stream::iter(vec![
                Ok(SseEvent::data("hello")),
                Ok(SseEvent::data("world")),
            ]));
            Ok(events)
        })
    }
    fn handle_websocket(
        &self,
        _: HttpRequest,
        _: RequestContext,
        _: WsChannel,
    ) -> BoxFuture<'_, HttpIngressResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

// ── WebSocket echo handler stub ───────────────────────────────────────────────

struct AxumStreamStubWsEchoHandler;
impl HttpStream for AxumStreamStubWsEchoHandler {
    fn handle_sse(
        &self,
        _: HttpRequest,
        _: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
        Box::pin(async { Ok(Box::pin(stream::empty()) as SseStream) })
    }
    fn handle_websocket(
        &self,
        _: HttpRequest,
        _: RequestContext,
        mut channel: WsChannel,
    ) -> BoxFuture<'_, HttpIngressResult<()>> {
        Box::pin(async move {
            use futures::StreamExt;
            while let Some(msg) = channel.receiver.next().await {
                match msg {
                    Ok(m) => {
                        let _ = channel.sender.send(m);
                    }
                    Err(_) => break,
                }
            }
            Ok(())
        })
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

async fn start_sse_server() -> (String, tokio::sync::oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let url = format!("http://{addr}/events");
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), Arc::new(AxumStreamStubHttpIngress))
        .with_stream_handler(Arc::new(AxumStreamStubSseHandler));
    tokio::spawn(async move {
        let signal = async move {
            let _ = shutdown_rx.await;
        };
        let _ = server.serve_with_listener(listener, signal).await;
    });
    (url, shutdown_tx)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_sse_request_returns_200_with_event_stream_content_type() {
    let (url, shutdown_tx) = start_sse_server().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Accept", "text/event-stream")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
        .expect("request failed");

    assert_eq!(resp.status(), 200);
    let ct = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.contains("text/event-stream"),
        "expected SSE content-type, got: {ct}"
    );

    let _ = shutdown_tx.send(());
}

#[tokio::test]
async fn test_non_sse_request_falls_through_to_regular_handler() {
    let (url, shutdown_tx) = start_sse_server().await;

    let resp = reqwest::get(&url).await.expect("request failed");

    assert_eq!(resp.status(), 200);
    let body = resp.bytes().await.unwrap();
    assert_eq!(body.as_ref(), b"ok");

    let _ = shutdown_tx.send(());
}

#[test]
fn test_axum_http_server_with_stream_handler_is_constructible() {
    let _s = AxumHttpServer::new("127.0.0.1:0", Arc::new(AxumStreamStubHttpIngress))
        .with_stream_handler(Arc::new(AxumStreamStubSseHandler));
}

#[test]
fn test_websocket_echo_handler_stub_is_constructible() {
    let _s = AxumHttpServer::new("127.0.0.1:0", Arc::new(AxumStreamStubHttpIngress))
        .with_stream_handler(Arc::new(AxumStreamStubWsEchoHandler));
}
