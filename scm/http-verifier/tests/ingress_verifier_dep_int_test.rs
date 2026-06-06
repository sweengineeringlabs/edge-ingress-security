//! Dep coverage for swe-edge-ingress-verifier.
use swe_edge_ingress_verifier::Claims;
/// @covers: swe-edge-ingress-verifier
#[test]
fn verifier_struct_ingress_verifier_dep_claims_accessible_int_test() {
    let _ = std::any::TypeId::of::<Claims>();
}
