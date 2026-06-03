//! Tests for sse_stream.
/// @covers: sse_stream
#[test]
fn transport_type_sse_stream_is_publicly_exported_int_test() {
    let _ = std::any::type_name::<swe_edge_ingress_http::SseStream>();
}
