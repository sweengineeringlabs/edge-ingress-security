//! Tests for http_body.
use swe_edge_ingress_http::HttpBody;
/// @covers: HttpBody
#[test]
fn transport_enum_http_body_raw_preserves_bytes_int_test() {
    let b = HttpBody::Raw(vec![1, 2, 3]);
    assert!(matches!(b, HttpBody::Raw(bytes) if bytes == [1, 2, 3]));
}
