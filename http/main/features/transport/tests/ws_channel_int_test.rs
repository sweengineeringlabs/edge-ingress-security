//! Tests for WsChannel.

use futures::stream;
use swe_edge_ingress_http::WsChannel;
use tokio::sync::mpsc;

#[test]
fn test_ws_channel_instantiable() {
    let (sender, _receiver) = mpsc::unbounded_channel();
    let _channel = WsChannel {
        sender,
        receiver: Box::pin(stream::empty()),
    };
}
