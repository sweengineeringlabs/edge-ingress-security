//! Tests for http_method.
/// @covers: http_method
#[test]
fn transport_enum_http_method_is_publicly_exported_int_test() {
    let _ = std::any::type_name::<swe_edge_ingress_http::HttpMethod>();
}
