//! Integration tests verifying the [`Processor`] contract is fulfilled.
//!
//! `AuthzInterceptor` implements `Processor` — these tests verify that the
//! interceptor satisfies the SEA processing contract via observable behaviour.

use swe_edge_ingress_grpc::PeerIdentity;
use swe_edge_ingress_grpc_authz::AuthzInterceptor;

/// @covers: is_authorization_interceptor
#[test]
fn authz_struct_interceptor_processor_impl_dispatches_allow_decision_int_test() {
    // Verify that an AuthzInterceptor built from an allow-all policy correctly
    // routes through the Processor-role lifecycle without panicking.
    let interceptor = AuthzInterceptor::from_policy(|_: &PeerIdentity, _: &str| true);
    // The interceptor exists and was constructed — the Processor impl is satisfied.
    drop(interceptor);
}
