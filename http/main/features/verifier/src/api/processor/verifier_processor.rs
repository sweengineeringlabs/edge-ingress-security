//! Interface counterpart for `core::processor::verifier_processor`.
/// Marker type for the verifier processor specification.
#[expect(
    dead_code,
    reason = "SEA api/ marker counterpart for core::processor::VerifierProcessor — names the spec; the concrete processor is built via VerifierSvc::processor()"
)]
pub struct VerifierProcessor;
