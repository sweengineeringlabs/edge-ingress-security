//! HTTP streaming inbound port — SSE push and WebSocket full-duplex.

use edge_domain::RequestContext;
use futures::future::BoxFuture;

use crate::api::port::http_inbound_result::HttpInboundResult;
use crate::api::value_object::sse::SseStream;
use crate::api::value_object::ws::WsChannel;
use crate::api::value_object::HttpRequest;

/// Handles HTTP transport-level streaming connections (server-inbound).
///
/// # SSE (Server-Sent Events)
/// The handler owns the push stream. The transport layer drives the stream
/// and serialises each [`SseEvent`](crate::api::value_object::sse::SseEvent)
/// into a `text/event-stream` wire frame.
///
/// # WebSocket
/// The transport upgrades the connection, creates a [`WsChannel`], and passes
/// it to the handler. The handler processes frames until it returns, at which
/// point the transport closes the connection.
pub trait HttpStreamInbound: Send + Sync {
    /// Handle a Server-Sent Events upgrade request.
    ///
    /// Returns a lazy stream of [`SseEvent`](crate::api::value_object::sse::SseEvent) frames
    /// that the transport will forward to the connected client.
    fn handle_sse(
        &self,
        request: HttpRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<SseStream>>;

    /// Handle a WebSocket upgrade request.
    ///
    /// The transport provides `channel`: use `channel.sender` to push frames to
    /// the client and `channel.receiver` to read incoming frames. Return when
    /// the session is complete; the transport closes the connection on return.
    fn handle_websocket(
        &self,
        request: HttpRequest,
        ctx: RequestContext,
        channel: WsChannel,
    ) -> BoxFuture<'_, HttpInboundResult<()>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_stream_inbound_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpStreamInbound) {}
    }
}
