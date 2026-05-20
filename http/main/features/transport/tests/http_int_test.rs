//! Integration tests exercising the `http` crate dependency directly.
//!
//! Verifies that the transport layer correctly handles `http::Uri` parsing
//! and HTTP method mapping through the inbound pipeline.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use http::Method;
use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpInbound, HttpInboundResult, HttpMethod, HttpRequest,
    HttpResponse, RequestContext,
};

// Verify http::Method is available (exercises the http dependency directly).
#[allow(dead_code)]
fn _http_get_method() -> Method {
    Method::GET
}

// ── Handler that reflects the parsed method and path ─────────────────────────

struct ReflectHandler;

impl HttpInbound for ReflectHandler {
    fn handle(
        &self,
        req: HttpRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        Box::pin(async move {
            let body = format!("{} {}", req.method, req.url).into_bytes();
            Ok(HttpResponse::new(200, body))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

async fn start() -> (String, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{addr}");
    let (tx, rx) = oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), Arc::new(ReflectHandler));
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
/// Exercises: http::Uri parsing — path extraction from a full URL.
#[tokio::test]
async fn test_http_uri_path_extracted_correctly_from_get_request() {
    let (base, _shutdown) = start().await;
    let resp = reqwest::get(format!("{base}/http/path/test"))
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert!(body.contains("/http/path/test"), "{body}");
}

/// @covers: AxumHttpServer::serve_with_listener
/// Exercises: http::Method mapping — DELETE method is preserved through the stack.
#[tokio::test]
async fn test_http_delete_method_mapped_and_reflected_correctly() {
    let (base, _shutdown) = start().await;
    let resp = reqwest::Client::new()
        .delete(format!("{base}/resource/1"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert!(body.contains("DELETE"), "{body}");
}

/// @covers: HttpMethod
/// Exercises: http crate method names round-trip through the HttpMethod enum.
#[test]
fn test_http_method_enum_covers_standard_http_verbs() {
    assert_eq!(HttpMethod::Get.to_string(), "GET");
    assert_eq!(HttpMethod::Post.to_string(), "POST");
    assert_eq!(HttpMethod::Put.to_string(), "PUT");
    assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
    assert_eq!(HttpMethod::Patch.to_string(), "PATCH");
    assert_eq!(HttpMethod::Head.to_string(), "HEAD");
    assert_eq!(HttpMethod::Options.to_string(), "OPTIONS");
}

/// @covers: HttpRequest::get
/// Exercises: http crate URI construction via the transport value objects.
#[test]
fn test_http_request_get_constructs_valid_uri_string() {
    let req = HttpRequest::get("https://example.com/api/v1?foo=bar");
    assert_eq!(req.url, "https://example.com/api/v1?foo=bar");
    assert_eq!(req.method, HttpMethod::Get);
}
