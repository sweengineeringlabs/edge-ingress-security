//! Tests for http_ingress_result.
use swe_edge_ingress_http::HttpIngressResult;
/// @covers: HttpIngressResult
#[test]
fn transport_type_http_ingress_result_is_accessible_int_test() {
    let _: HttpIngressResult<()> = Ok(());
}
