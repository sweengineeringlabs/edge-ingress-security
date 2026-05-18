//! WebSocket send-side channel type.

use tokio::sync::mpsc;

use crate::api::value_object::ws::ws_message::WsMessage;

/// The send half of a [`WsChannel`](super::ws_channel::WsChannel).
///
/// The server implementation pushes [`WsMessage`] frames into this sender;
/// the transport layer forwards them to the connected WebSocket peer.
pub type WsSender = mpsc::UnboundedSender<WsMessage>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_sender_can_be_constructed_from_mpsc_channel() {
        let (tx, _rx) = mpsc::unbounded_channel::<WsMessage>();
        let _: WsSender = tx;
    }
}
