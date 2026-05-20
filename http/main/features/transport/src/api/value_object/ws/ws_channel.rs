//! WebSocket full-duplex channel value object.

use crate::api::value_object::ws::ws_receiver::WsReceiver;
use crate::api::value_object::ws::ws_sender::WsSender;

/// A full-duplex WebSocket channel returned after a successful handshake.
///
/// The server implementation receives this struct from [`HttpStreamInbound::handle_websocket`]
/// and uses [`sender`] to push frames to the peer while consuming
/// incoming frames from [`receiver`].
///
/// [`HttpStreamInbound::handle_websocket`]: crate::api::port::http::http_stream_inbound::HttpStreamInbound::handle_websocket
pub struct WsChannel {
    /// Send frames to the connected WebSocket peer.
    pub sender: WsSender,
    /// Receive frames from the connected WebSocket peer.
    pub receiver: WsReceiver,
}

#[cfg(test)]
mod tests {
    use futures::stream;
    use tokio::sync::mpsc;

    use super::*;

    #[test]
    fn test_ws_channel_can_be_constructed() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let _ch = WsChannel {
            sender: tx,
            receiver: Box::pin(stream::empty()),
        };
    }
}
