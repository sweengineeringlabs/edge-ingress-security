//! HTTP server port trait.

use futures::future::BoxFuture;

use crate::api::port::http_ingress_error::HttpIngressError;

/// A runnable HTTP server that drives an [`HttpIngress`](crate::api::port::http_ingress::HttpIngress) handler.
pub trait HttpServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpIngressError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_server_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpServer) {}
    }
}
