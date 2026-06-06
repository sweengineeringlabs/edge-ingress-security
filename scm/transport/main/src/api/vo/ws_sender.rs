//! WebSocket send-side channel type.

use tokio::sync::mpsc;

use crate::api::vo::ws_message::WsMessage;

/// The send half of a [`WsChannel`](super::ws_channel::WsChannel).
///
/// The server implementation pushes [`WsMessage`] frames into this sender;
/// the transport layer forwards them to the connected WebSocket peer.
pub type WsSender = mpsc::UnboundedSender<WsMessage>;
