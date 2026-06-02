//! Tests for http_encode_fn.
use swe_edge_ingress_http_transport::HttpEncodeFn;
/// @covers: HttpEncodeFn
#[test]
fn transport_struct_http_encode_fn_is_accessible_int_test() {
    let _ = std::any::TypeId::of::<HttpEncodeFn>();
}
