//! WebSocket receive-side stream type.

use std::pin::Pin;

use futures::Stream;

use crate::api::error::HttpIngressError;
use crate::api::value::ws::ws_message::WsMessage;

/// The receive half of a [`WsChannel`](super::ws_channel::WsChannel).
///
/// Yields [`WsMessage`] frames arriving from the connected WebSocket peer.
/// Exhausted when the peer closes the connection.
pub type WsReceiver = Pin<Box<dyn Stream<Item = Result<WsMessage, HttpIngressError>> + Send>>;
