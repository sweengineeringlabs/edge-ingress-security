//! Integration tests verifying SEA interface contracts from `api/traits.rs`.

use swe_edge_ingress_grpc::PeerIdentity;
use swe_edge_ingress_grpc_authz::AuthzInterceptor;

/// @covers: is_authorization_interceptor
#[test]
fn authz_struct_interceptor_satisfies_processor_contract_int_test() {
    // AuthzInterceptor implements Processor — verify the type can be
    // constructed and used without trait visibility in the public API.
    let interceptor = AuthzInterceptor::from_policy(|_: &PeerIdentity, _: &str| true);
    drop(interceptor);
}
