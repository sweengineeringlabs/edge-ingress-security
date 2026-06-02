//! Tests for registry_dispatcher_impl.
use swe_edge_ingress_http_transport::HttpHandlerRegistryDispatcher;
/// @covers: HttpHandlerRegistryDispatcher
#[test]
fn transport_struct_registry_dispatcher_impl_is_accessible_int_test() {
    let _ = std::any::TypeId::of::<HttpHandlerRegistryDispatcher>();
}
