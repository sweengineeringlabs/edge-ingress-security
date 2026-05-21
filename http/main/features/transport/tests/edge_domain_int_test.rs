//! Integration tests exercising the `edge-domain` dependency directly.
//!
//! Verifies that `edge_domain` types (Handler, HandlerRegistry, RequestContext)
//! interact correctly with the transport crate's dispatcher.

use std::sync::Arc;

use edge_domain::{Handler, HandlerError, HandlerRegistry, RequestContext};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use swe_edge_ingress_http::{
    AxumHttpServer, HttpHandlerAdapter, HttpHandlerRegistryDispatcher, HttpInbound,
    HttpInboundError, HttpRequest, HttpResponse,
};

// ── Domain handler under test ─────────────────────────────────────────────────

#[derive(Debug)]
struct GreetReq {
    name: String,
}

#[derive(Debug)]
struct GreetResp {
    greeting: String,
}

struct GreetHandler;

#[async_trait::async_trait]
impl Handler<GreetReq, GreetResp> for GreetHandler {
    fn id(&self) -> &str {
        "greet"
    }
    fn pattern(&self) -> &str {
        "/greet"
    }
    async fn execute(&self, req: GreetReq) -> Result<GreetResp, HandlerError> {
        Ok(GreetResp {
            greeting: format!("hello, {}!", req.name),
        })
    }
}

fn decode(req: &HttpRequest) -> Result<GreetReq, HttpInboundError> {
    let name = req
        .query
        .get("name")
        .cloned()
        .unwrap_or_else(|| "world".to_string());
    Ok(GreetReq { name })
}

fn encode(resp: GreetResp) -> HttpResponse {
    HttpResponse::new(200, resp.greeting.into_bytes())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_dispatcher() -> HttpHandlerRegistryDispatcher {
    let registry = Arc::new(HandlerRegistry::new());
    let dispatcher = HttpHandlerRegistryDispatcher::new(Arc::clone(&registry));
    let adapter = HttpHandlerAdapter::new(Arc::new(GreetHandler), decode, encode);
    dispatcher.register(adapter).unwrap();
    dispatcher
}

async fn start_dispatcher_server(handler: Arc<dyn HttpInbound>) -> (String, oneshot::Sender<()>) {
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

// ── Tests ─────────────────────────────────────────────────────────────────────

/// @covers: HttpHandlerRegistryDispatcher::handle
/// Exercises: edge_domain HandlerRegistry, Handler trait, RequestContext.
#[tokio::test]
async fn test_edge_domain_handler_registry_routes_request_to_handler() {
    let dispatcher = build_dispatcher();
    let req = HttpRequest::get("/greet").with_query("name", "alice");
    let resp = dispatcher
        .handle(req, RequestContext::unauthenticated())
        .await
        .unwrap();
    assert_eq!(resp.status, 200);
    let body = String::from_utf8(resp.body).unwrap();
    assert!(body.contains("alice"), "{body}");
}

/// @covers: HttpHandlerRegistryDispatcher::handle
/// Exercises: edge_domain NotFound path when no handler matches.
#[tokio::test]
async fn test_edge_domain_dispatcher_returns_not_found_for_unregistered_route() {
    let dispatcher = build_dispatcher();
    let err = dispatcher
        .handle(
            HttpRequest::get("/unknown"),
            RequestContext::unauthenticated(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, HttpInboundError::NotFound(_)));
}

/// @covers: HttpHandlerRegistryDispatcher::handle
/// Exercises: edge_domain handler pipeline through the HTTP server end-to-end.
#[tokio::test]
async fn test_edge_domain_handler_served_over_axum_returns_200() {
    let dispatcher = Arc::new(build_dispatcher());
    let (base, _shutdown) = start_dispatcher_server(dispatcher).await;

    let resp = reqwest::get(format!("{base}/greet?name=bob"))
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert!(body.contains("bob"), "{body}");
}

/// @covers: HttpHandlerAdapter::new
/// Exercises: edge_domain decode/encode function types end-to-end.
#[tokio::test]
async fn test_edge_domain_handler_adapter_decode_encode_roundtrip() {
    let adapter: Arc<dyn Handler<HttpRequest, HttpResponse>> = Arc::new(HttpHandlerAdapter::new(
        Arc::new(GreetHandler),
        decode,
        encode,
    ));
    let mut req = HttpRequest::get("/greet");
    req.query.insert("name".into(), "carol".into());
    let resp = adapter
        .execute_with_context(req, RequestContext::unauthenticated())
        .await
        .unwrap();
    assert_eq!(resp.status, 200);
    let body = String::from_utf8(resp.body).unwrap();
    assert!(body.contains("carol"), "{body}");
}
