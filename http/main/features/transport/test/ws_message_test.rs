//! Tests for WsMessage.

use swe_edge_ingress_http::WsMessage;

#[test]
fn test_ws_message_instantiable() {
    let _message = WsMessage::text("test");
}
