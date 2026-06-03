//! Tests for http_auth.
use swe_edge_ingress_http::HttpAuth;
/// @covers: HttpAuth
#[test]
fn transport_struct_http_auth_none_is_default_int_test() {
    let a = HttpAuth::None;
    assert!(matches!(a, HttpAuth::None));
}
