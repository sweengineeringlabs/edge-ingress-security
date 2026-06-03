//! Integration tests for `TransportSvc`.
use swe_edge_ingress_http::TransportSvc;

/// @covers: TransportSvc::create_config_builder
#[test]
fn transport_struct_transport_svc_create_config_builder_returns_seeded_builder_int_test() {
    let b = TransportSvc::create_config_builder();
    assert!(!b.name().is_empty());
}
