//! HTTP server port trait.

use futures::future::BoxFuture;

use crate::api::port::http_inbound_error::HttpInboundError;

/// A runnable HTTP server that drives an [`HttpInbound`](crate::api::port::http_inbound::HttpInbound) handler.
pub trait HttpServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpInboundError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_server_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpServer) {}
    }
}
