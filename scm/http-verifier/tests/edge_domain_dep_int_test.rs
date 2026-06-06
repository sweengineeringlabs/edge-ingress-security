//! Dep coverage for edge-domain.
use edge_domain::RequestContext;
/// @covers: edge-domain
#[test]
fn verifier_struct_edge_domain_dep_request_context_accessible_int_test() {
    let _ = std::any::TypeId::of::<RequestContext>();
}
