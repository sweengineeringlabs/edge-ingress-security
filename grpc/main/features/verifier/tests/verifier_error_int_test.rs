//! Integration tests for `VerifierError`.

use swe_edge_ingress_grpc_verifier::VerifierError;

/// @covers: VerifierError
#[test]
fn verifier_struct_verifier_error_is_publicly_exported_int_test() {
    let _ = std::any::type_name::<VerifierError>();
}
