//! Dep coverage for axum.
use axum::http::StatusCode;
/// @covers: axum
#[test]
fn verifier_struct_axum_dep_status_code_is_accessible_int_test() {
    assert_eq!(StatusCode::OK.as_u16(), 200);
}
