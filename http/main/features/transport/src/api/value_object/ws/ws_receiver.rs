//! WebSocket receive-side stream type.

use std::pin::Pin;

use futures::Stream;

use crate::api::port::http_inbound_error::HttpInboundError;
use crate::api::value_object::ws::ws_message::WsMessage;

/// The receive half of a [`WsChannel`](super::ws_channel::WsChannel).
///
/// Yields [`WsMessage`] frames arriving from the connected WebSocket peer.
/// Exhausted when the peer closes the connection.
pub type WsReceiver = Pin<Box<dyn Stream<Item = Result<WsMessage, HttpInboundError>> + Send>>;

#[cfg(test)]
mod tests {
    use futures::stream;

    use super::*;

    #[test]
    fn test_ws_receiver_empty_stream_is_valid() {
        let _r: WsReceiver = Box::pin(stream::empty());
    }
}
