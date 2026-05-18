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
/// The transport upgrades the connection and hands a full-duplex
/// [`WsChannel`] to the handler. The handler may send and receive frames
/// concurrently; the connection stays open until the channel is dropped.
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
    /// Returns a [`WsChannel`] after the handshake completes. The handler
    /// owns the channel and may send or receive frames at will.
    fn handle_websocket(
        &self,
        request: HttpRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<WsChannel>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_stream_inbound_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpStreamInbound) {}
    }
}
