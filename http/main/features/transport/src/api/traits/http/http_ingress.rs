//! HTTP inbound trait — handles incoming HTTP requests.

use edge_domain::RequestContext;
use futures::future::BoxFuture;

use crate::api::types::http_health_check::HttpHealthCheck;
use crate::api::types::http_ingress_result::HttpIngressResult;
use crate::api::value::{HttpRequest, HttpResponse};

/// Receives and handles inbound HTTP requests.
pub trait HttpIngress: Send + Sync {
    /// Handle an HTTP request with its per-request context and return a response.
    ///
    /// `ctx` carries the authenticated identity, tenant, and trace metadata
    /// extracted by the ingress middleware stack before dispatch.
    fn handle(
        &self,
        request: HttpRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>>;
    /// Perform a health check of this handler.
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>>;
}
