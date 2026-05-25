//! Tests for BearerService.

use swe_edge_ingress_http_verifier::BearerService;

#[test]
fn test_bearer_service_exists() {
    let _ = std::any::type_name::<BearerService>();
}
