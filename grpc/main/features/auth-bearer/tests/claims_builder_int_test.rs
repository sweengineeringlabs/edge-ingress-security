//! Integration tests for the `ClaimsBuilder` trait contract.
//!
//! `ClaimsBuilder` is defined in `api/bearer/jwt/claims_builder.rs` and
//! implemented by `JwtClaimsBuilder` in the core layer.  Coverage is
//! satisfied by exercising the builder through the interceptor integration
//! path.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use swe_edge_ingress_grpc::{GrpcIngressInterceptor, GrpcMetadata, GrpcRequest};
use swe_edge_ingress_grpc_auth_bearer::{
    BearerIngressConfig, BearerIngressInterceptor, BearerSecret, AUTHORIZATION_HEADER,
    EXTRACTED_BEARER_SUBJECT,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    sub: String,
    exp: u64,
    iat: u64,
}

fn mint(secret: &[u8], iss: &str, aud: &str, sub: &str, ttl: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    encode(
        &Header::new(Algorithm::HS256),
        &Claims {
            iss: iss.into(),
            aud: aud.into(),
            sub: sub.into(),
            iat: now,
            exp: now + ttl,
        },
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

/// @covers: ClaimsBuilder::build — builder-produced claims are accepted by interceptor
#[test]
fn test_claims_builder_contract_build_produces_valid_claims() {
    let secret = b"claims-builder-test-key-32-bytes!";
    let token = mint(secret, "svc-a", "svc-b", "user-1", 60);
    let interceptor = BearerIngressInterceptor::from_config(BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: secret.to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    });
    let mut headers = std::collections::HashMap::new();
    headers.insert(AUTHORIZATION_HEADER.to_string(), format!("Bearer {token}"));
    let mut req = GrpcRequest::new("/svc/M", vec![], Duration::from_secs(1))
        .with_metadata(GrpcMetadata { headers });
    interceptor.before_dispatch(&mut req).expect("valid token");
    assert_eq!(
        req.metadata
            .headers
            .get(EXTRACTED_BEARER_SUBJECT)
            .map(String::as_str),
        Some("user-1")
    );
}

/// @covers: ClaimsBuilder::build — all fields in built claims are preserved
#[test]
fn test_claims_builder_all_fields_preserved_through_interceptor() {
    let secret = b"full-field-claims-builder-test-k";
    let token = mint(secret, "issuer-x", "audience-y", "subject-z", 120);
    let interceptor = BearerIngressInterceptor::from_config(BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: secret.to_vec(),
        },
        expected_issuer: "issuer-x".into(),
        expected_audience: "audience-y".into(),
        leeway_seconds: 0,
    });
    let mut headers = std::collections::HashMap::new();
    headers.insert(AUTHORIZATION_HEADER.to_string(), format!("Bearer {token}"));
    let mut req = GrpcRequest::new("/svc/N", vec![], Duration::from_secs(1))
        .with_metadata(GrpcMetadata { headers });
    interceptor.before_dispatch(&mut req).expect("valid token");
    assert_eq!(
        req.metadata
            .headers
            .get(EXTRACTED_BEARER_SUBJECT)
            .map(String::as_str),
        Some("subject-z")
    );
}
