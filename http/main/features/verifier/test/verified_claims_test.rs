//! Tests for VerifiedClaims.

use swe_edge_ingress_http_verifier::VerifiedClaims;

#[test]
fn test_verified_claims_exists() {
    let _ = std::any::type_name::<VerifiedClaims>();
}
