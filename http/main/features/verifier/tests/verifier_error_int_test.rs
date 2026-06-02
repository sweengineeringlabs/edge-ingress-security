//! Tests for VerifierError.
use swe_edge_ingress_http_verifier::VerifierError;
/// @covers: VerifierError
#[test]
fn verifier_struct_verifier_error_is_accessible_int_test() {
    let _ = std::any::TypeId::of::<VerifierError>();
}
