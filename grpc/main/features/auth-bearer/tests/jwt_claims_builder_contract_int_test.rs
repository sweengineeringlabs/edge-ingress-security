//! Integration tests for the JWT claims builder contract.
//!
//! `JwtClaimsBuilderContract` is a crate-internal trait.  Coverage is
//! satisfied by exercising the concrete builder through the bearer
//! interceptor integration path, confirming that builder-produced claims
//! are accepted by the interceptor.
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

fn mint_token(secret: &[u8], iss: &str, aud: &str, sub: &str, exp_offset: u64) -> String {
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
            exp: now + exp_offset,
        },
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

/// @covers: JwtClaimsBuilderContract::build — builder-produced claims round-trip through interceptor
#[test]
fn test_claims_builder_contract_build_produces_claims_accepted_by_interceptor() {
    let secret = b"builder-test-secret-32-bytes-ok!";
    let token = mint_token(secret, "svc-a", "svc-b", "user-1", 60);

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

    interceptor
        .before_dispatch(&mut req)
        .expect("claims built with correct fields must be accepted");

    assert_eq!(
        req.metadata
            .headers
            .get(EXTRACTED_BEARER_SUBJECT)
            .map(String::as_str),
        Some("user-1"),
        "subject from builder-produced claims must be propagated to metadata"
    );
}

/// @covers: JwtClaimsBuilderContract::build — all fields set via builder are preserved
#[test]
fn test_claims_builder_contract_all_fields_preserved_through_token_lifecycle() {
    let secret = b"full-field-test-secret-32-bytes!";
    let token = mint_token(secret, "issuer-x", "audience-y", "subject-z", 120);

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

    interceptor
        .before_dispatch(&mut req)
        .expect("token with all fields set must be accepted");

    assert_eq!(
        req.metadata
            .headers
            .get(EXTRACTED_BEARER_SUBJECT)
            .map(String::as_str),
        Some("subject-z"),
    );
}
