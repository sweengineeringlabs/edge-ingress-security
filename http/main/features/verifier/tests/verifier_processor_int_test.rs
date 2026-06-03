//! Tests for `VerifierProcessor`, the concrete `Processor` the SAF builds.
//!
//! `VerifierProcessor` is `pub(crate)` (SEA keeps implementations out of the
//! public facade), so it is exercised through `VerifierSvc::processor()`,
//! which constructs and returns it as `impl Processor`. This is a compile
//! contract over that construction path.

use swe_edge_ingress_http_verifier::VerifierSvc;

/// @covers: VerifierProcessor
#[test]
fn verifier_struct_verifier_processor_constructed_by_saf_int_test() {
    let _processor = VerifierSvc::processor();
}
