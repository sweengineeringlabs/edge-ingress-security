//! Tests for BearerLayer.

use swe_edge_ingress_http_verifier::BearerLayer;

#[test]
fn test_bearer_layer_exists() {
    let _ = std::any::type_name::<BearerLayer>();
}
