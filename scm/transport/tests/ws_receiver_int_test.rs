//! Tests for ws_receiver.
/// @covers: ws_receiver
#[test]
fn transport_type_ws_receiver_is_publicly_exported_int_test() {
    let _ = std::any::type_name::<swe_edge_ingress_http::WsReceiver>();
}
