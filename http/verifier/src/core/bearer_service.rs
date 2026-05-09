//! `BearerService` — tower `Service` implementation for JWT bearer verification.
//!
//! Auth failures are converted to `401 Unauthorized` responses; the service
//! error type is `Infallible` so it composes cleanly with `axum::Router::layer`.

use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use tower::{Layer, Service};

use crate::api::auth_error::HttpAuthError;
use crate::api::bearer_layer::BearerLayer;
use crate::api::bearer_service::BearerService;
use crate::api::verified_claims::VerifiedClaims;

impl<S> Layer<S> for BearerLayer {
    type Service = BearerService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        BearerService { inner, verifier: Arc::clone(&self.verifier) }
    }
}

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

impl<S> Service<Request<Body>> for BearerService<S>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let verifier = Arc::clone(&self.verifier);
        let mut inner = self.inner.clone();

        Box::pin(async move {
            match extract_bearer(&req) {
                Err(e) => Ok(auth_error_response(e)),
                Ok(token) => match verifier.verify(token) {
                    Err(e) => Ok(auth_error_response(HttpAuthError::InvalidToken(e))),
                    Ok(claims) => {
                        req.extensions_mut().insert(VerifiedClaims(claims));
                        inner.call(req).await
                    }
                },
            }
        })
    }
}

fn extract_bearer<'a>(req: &'a Request<Body>) -> Result<&'a str, HttpAuthError> {
    let header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(HttpAuthError::MissingAuthorization)?
        .to_str()
        .map_err(|_| HttpAuthError::MalformedAuthorization)?;

    header
        .strip_prefix("Bearer ")
        .ok_or(HttpAuthError::MalformedAuthorization)
}

fn auth_error_response(err: HttpAuthError) -> Response<Body> {
    tracing::debug!(?err, "bearer auth rejected request");
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from(err.to_string()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::{routing::get, Router};
    use swe_edge_ingress_verifier::{Claims, VerifierError};
    use tower::ServiceExt;

    struct AlwaysOk;
    impl TokenVerifier for AlwaysOk {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(serde_json::from_str(r#"{"sub":"alice"}"#).unwrap())
        }
    }

    struct AlwaysFail;
    impl TokenVerifier for AlwaysFail {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("bad".into()))
        }
    }

    fn router(verifier: Arc<dyn TokenVerifier>) -> Router {
        Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(BearerLayer::new(verifier))
    }

    async fn call(verifier: Arc<dyn TokenVerifier>, req: Request<Body>) -> Response<Body> {
        router(verifier).oneshot(req).await.unwrap()
    }

    /// @covers: BearerService — valid token passes through with 200.
    #[tokio::test]
    async fn test_bearer_service_valid_token_passes_through() {
        let req = Request::builder()
            .header("Authorization", "Bearer valid-token")
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = call(Arc::new(AlwaysOk), req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// @covers: BearerService — missing Authorization header returns 401.
    #[tokio::test]
    async fn test_bearer_service_missing_header_returns_401() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let resp = call(Arc::new(AlwaysOk), req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    /// @covers: BearerService — malformed Authorization header returns 401.
    #[tokio::test]
    async fn test_bearer_service_malformed_header_returns_401() {
        let req = Request::builder()
            .header("Authorization", "Basic dXNlcjpwYXNz")
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = call(Arc::new(AlwaysOk), req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    /// @covers: BearerService — rejected token returns 401.
    #[tokio::test]
    async fn test_bearer_service_invalid_token_returns_401() {
        let req = Request::builder()
            .header("Authorization", "Bearer bad-token")
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = call(Arc::new(AlwaysFail), req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
