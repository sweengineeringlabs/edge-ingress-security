//! HTTP server port — contract for a runnable HTTP inbound server.

use futures::future::BoxFuture;

use crate::api::port::http_inbound::HttpInboundError;

/// A runnable HTTP server that drives an [`HttpInbound`](super::port::http_inbound::HttpInbound) handler.
pub trait HttpServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpInboundError>>;
}
