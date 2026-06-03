//! Tests for the `Validator` contract surface.
//!
//! `Validator` is an internal trait (SEA rule 126 keeps traits out of the
//! public facade) and this crate currently exposes no validatable public
//! type, so the contract is reachable only through `VerifierSvc::validate`.
//! This compile contract guards that the facade type hosting validation stays
//! publicly exported; the behavioural contract of `Validator` is covered by
//! the inline unit tests of its implementors.

use swe_edge_ingress_http_verifier::VerifierSvc;

/// @covers: Validator
#[test]
fn verifier_trait_validator_host_facade_is_public_int_test() {
    let _ = std::any::type_name::<VerifierSvc>();
}
