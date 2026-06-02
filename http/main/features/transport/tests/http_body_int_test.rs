//! Tests for http_body.
use swe_edge_ingress_http_transport::HttpBody;
/// @covers: HttpBody
#[test]
fn transport_struct_http_body_empty_constructs_int_test() {
    let b = HttpBody::Empty;
    assert!(matches!(b, HttpBody::Empty));
}
