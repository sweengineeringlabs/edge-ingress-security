//! Integration tests for the TokenVerifier trait.

use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier, TokenVerifier};

fn hs256_verifier(secret: &[u8]) -> JwtVerifier {
    let cfg = JwtConfig {
        key: JwtKey::Hs256 {
            secret: secret.to_vec(),
        },
        required_issuer: None,
        required_audience: None,
        leeway_seconds: 0,
    };
    JwtVerifier::from_config(&cfg).expect("JwtVerifier::from_config")
}

/// @covers: TokenVerifier
#[test]
fn test_token_verifier_rejects_invalid_token() {
    let v = hs256_verifier(b"secret");
    assert!(v.verify("not.a.token").is_err());
}
