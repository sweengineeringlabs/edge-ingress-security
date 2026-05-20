//! Integration tests exercising the `swe-edge-ingress-verifier` dependency directly.
//!
//! Verifies that bearer-auth verification is wired correctly through
//! `AxumHttpServer::with_bearer_auth` and the `TokenVerifier` trait.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpInbound, HttpInboundResult, HttpRequest, HttpResponse,
    RequestContext,
};
use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

// ── Stub handler ─────────────────────────────────────────────────────────────

struct WhoAmIHandler;

impl HttpInbound for WhoAmIHandler {
    fn handle(
        &self,
        _req: HttpRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        let subject = ctx.subject.as_deref().unwrap_or("anonymous").to_string();
        Box::pin(async move {
            let body = format!("subject={subject}").into_bytes();
            Ok(HttpResponse::new(200, body))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

// ── Stub verifier ─────────────────────────────────────────────────────────────

struct StaticVerifier {
    subject: String,
}

impl TokenVerifier for StaticVerifier {
    fn verify(&self, token: &str) -> Result<Claims, VerifierError> {
        if token == "valid-token" {
            let json = format!(r#"{{"sub":"{}"}}"#, self.subject);
            Ok(serde_json::from_str(&json).unwrap())
        } else {
            Err(VerifierError::Invalid("bad token".to_string()))
        }
    }
}

// ── Helper ─────────────────────────────────────────────────────────────────────

async fn start_with_auth(subject: &str) -> (String, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{addr}");
    let (tx, rx) = oneshot::channel::<()>();

    let verifier = Arc::new(StaticVerifier {
        subject: subject.to_string(),
    });
    let server =
        AxumHttpServer::new(addr.to_string(), Arc::new(WhoAmIHandler)).with_bearer_auth(verifier);

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

/// @covers: AxumHttpServer::with_bearer_auth
/// Exercises: swe_edge_ingress_verifier TokenVerifier — valid token accepted.
#[tokio::test]
async fn test_swe_edge_ingress_verifier_valid_token_returns_200() {
    let (base, _shutdown) = start_with_auth("user-alice").await;
    let resp = reqwest::Client::new()
        .get(format!("{base}/whoami"))
        .header("Authorization", "Bearer valid-token")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

/// @covers: AxumHttpServer::with_bearer_auth
/// Exercises: swe_edge_ingress_verifier TokenVerifier — missing token rejected with 401.
#[tokio::test]
async fn test_swe_edge_ingress_verifier_missing_token_returns_401() {
    let (base, _shutdown) = start_with_auth("user-bob").await;
    let resp = reqwest::Client::new()
        .get(format!("{base}/whoami"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}

/// @covers: AxumHttpServer::with_bearer_auth
/// Exercises: swe_edge_ingress_verifier TokenVerifier — invalid token rejected with 401.
#[tokio::test]
async fn test_swe_edge_ingress_verifier_invalid_token_returns_401() {
    let (base, _shutdown) = start_with_auth("user-carol").await;
    let resp = reqwest::Client::new()
        .get(format!("{base}/whoami"))
        .header("Authorization", "Bearer wrong-token")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}
