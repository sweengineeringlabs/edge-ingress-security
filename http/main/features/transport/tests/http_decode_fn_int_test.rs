//! Tests for http_decode_fn.
use swe_edge_ingress_http::HttpDecodeFn;
/// @covers: HttpDecodeFn
#[test]
fn transport_struct_http_decode_fn_is_accessible_int_test() {
    let _ = std::any::type_name::<HttpDecodeFn<()>>();
}
