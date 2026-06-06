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
use axum::http::{Request, Response};
use tower::{Layer, Service};

use edge_domain::RequestContext;

use crate::api::error::HttpAuthError;
use crate::api::vo::verified_claims::VerifiedClaims;
use crate::spi::bearer::axum::bearer_layer::BearerLayer;
use crate::spi::bearer::axum::bearer_service::BearerService;
use crate::spi::bearer::axum::bearer_service_helper::BearerServiceHelper;

impl<S> Layer<S> for BearerLayer {
    type Service = BearerService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        BearerService {
            inner,
            verifier: Arc::clone(&self.verifier),
        }
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
            match BearerServiceHelper::extract_bearer(&req) {
                Err(e) => Ok(BearerServiceHelper::auth_error_response(e)),
                Ok(token) => match verifier.verify(token) {
                    Err(e) => Ok(BearerServiceHelper::auth_error_response(
                        HttpAuthError::from(e),
                    )),
                    Ok(claims) => {
                        // Build RequestContext from verified claims and insert for downstream.
                        let ctx = RequestContext::authenticated(
                            claims.sub.clone().unwrap_or_default(),
                            claims.iss.clone(),
                            claims
                                .custom
                                .get("tenant_id")
                                .map(|v| v.to_string().trim_matches('"').to_string()),
                            claims
                                .custom
                                .iter()
                                .map(|(k, v)| (k.clone(), v.to_string()))
                                .collect(),
                        );
                        req.extensions_mut().insert(ctx);
                        req.extensions_mut().insert(VerifiedClaims(claims));
                        inner.call(req).await
                    }
                },
            }
        })
    }
}
