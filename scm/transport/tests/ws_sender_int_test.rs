//! Tests for ws_sender.
/// @covers: ws_sender
#[test]
fn transport_type_ws_sender_is_publicly_exported_int_test() {
    let _ = std::any::type_name::<swe_edge_ingress_http::WsSender>();
}
