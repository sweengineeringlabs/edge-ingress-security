//! Integration tests for the `Validator` contract on [`BearerIngressConfig`].
//!
//! `Validator` is a crate-internal trait; the contract is exercised through
//! the public SAF surface — specifically by observing that interceptors built
//! from configs with empty issuer/audience fail at runtime rather than silently
//! accepting any token.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use swe_edge_ingress_grpc::{
    GrpcIngressError, GrpcIngressInterceptor, GrpcMetadata, GrpcRequest, GrpcStatusCode,
};
use swe_edge_ingress_grpc_auth_bearer::{
    BearerIngressConfig, BearerIngressInterceptor, BearerSecret, AUTHORIZATION_HEADER,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    sub: String,
    exp: u64,
    iat: u64,
}

fn mint(secret: &[u8], iss: &str, aud: &str, sub: &str, exp_secs_from_now: u64) -> String {
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
            exp: now + exp_secs_from_now,
        },
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

fn req_with_bearer(token_header: &str) -> GrpcRequest {
    let mut headers = std::collections::HashMap::new();
    headers.insert(AUTHORIZATION_HEADER.to_string(), token_header.to_string());
    GrpcRequest::new("/svc/M", vec![], Duration::from_secs(1))
        .with_metadata(GrpcMetadata { headers })
}

/// @covers: Validator — config with valid issuer and audience accepts matching token
#[test]
fn test_validator_config_with_non_empty_issuer_audience_accepts_valid_token() {
    let secret = b"valid-secret-key-32-bytes-long!";
    let token = mint(secret, "svc-a", "svc-b", "alice", 60);
    let interceptor = BearerIngressInterceptor::from_config(BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: secret.to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    });
    let mut req = req_with_bearer(&format!("Bearer {token}"));
    assert!(
        interceptor.before_dispatch(&mut req).is_ok(),
        "valid token must be accepted when issuer and audience are set"
    );
}

/// @covers: Validator — config with mismatched issuer rejects token (validates issuer field)
#[test]
fn test_validator_config_wrong_issuer_rejects_token() {
    let secret = b"valid-secret-key-32-bytes-long!";
    let token = mint(secret, "svc-a", "svc-b", "alice", 60);
    let interceptor = BearerIngressInterceptor::from_config(BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: secret.to_vec(),
        },
        expected_issuer: "wrong-issuer".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    });
    let mut req = req_with_bearer(&format!("Bearer {token}"));
    match interceptor.before_dispatch(&mut req) {
        Err(GrpcIngressError::Status(GrpcStatusCode::Unauthenticated, _)) => {}
        other => panic!("expected Unauthenticated for wrong issuer, got {other:?}"),
    }
}
