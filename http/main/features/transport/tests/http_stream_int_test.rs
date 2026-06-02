//! Tests for http_stream.
use swe_edge_ingress_http_transport::HttpStream;
/// @covers: HttpStream
#[test]
fn transport_trait_http_stream_is_object_safe_int_test() {
    fn _assert(_: &dyn HttpStream) {}
}
