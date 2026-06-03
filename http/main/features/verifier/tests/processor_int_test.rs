//! Tests for the `Processor` contract via the public SAF factory.
//!
//! `Processor` is an internal trait (SEA rule 126 keeps traits out of the
//! public facade), so it is exercised through `VerifierSvc::processor()` —
//! the only public surface that builds a `Processor`. This is a compile
//! contract: it fails to build if the factory or the trait wiring regresses.

use swe_edge_ingress_http_verifier::VerifierSvc;

/// @covers: Processor
#[test]
fn verifier_trait_processor_built_by_saf_factory_int_test() {
    let _processor = VerifierSvc::processor();
}
