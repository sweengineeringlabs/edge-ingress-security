//! HTTP server port trait.

use futures::future::BoxFuture;

use crate::api::port::http_ingress_error::HttpIngressError;

/// A runnable HTTP server that drives an [`HttpIngress`](crate::api::port::http_ingress::HttpIngress) handler.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub trait HttpServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpIngressError>>;
}
