//! WebSocket full-duplex channel value object.

use crate::api::value::ws::ws_receiver::WsReceiver;
use crate::api::value::ws::ws_sender::WsSender;

/// A full-duplex WebSocket channel returned after a successful handshake.
///
/// The server implementation receives this struct from [`HttpStream::handle_websocket`]
/// and uses [`sender`] to push frames to the peer while consuming
/// incoming frames from [`receiver`].
///
/// [`HttpStream::handle_websocket`]: crate::api::traits::http_stream::HttpStream::handle_websocket
pub struct WsChannel {
    /// Send frames to the connected WebSocket peer.
    pub sender: WsSender,
    /// Receive frames from the connected WebSocket peer.
    pub receiver: WsReceiver,
}
