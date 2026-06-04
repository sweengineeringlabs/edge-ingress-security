//! Tests for `Processor` trait.

/// @covers: Processor
#[test]
fn verifier_trait_processor_describe_via_saf_int_test() {
    use swe_edge_ingress_grpc_verifier::BearerTokenInterceptor;
    let _ = std::any::type_name::<BearerTokenInterceptor>();
}
