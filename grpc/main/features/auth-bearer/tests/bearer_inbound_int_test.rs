//! Integration tests for [`BearerInboundInterceptor`] — exercises the
//! crate's public facade exactly the way a consumer would.
//!
//! Tokens are minted directly via `jsonwebtoken` so this test has no
//! dependency on the egress auth-bearer crate.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use swe_edge_ingress_grpc::{
    GrpcInboundError, GrpcInboundInterceptor, GrpcMetadata, GrpcRequest, GrpcStatusCode,
};
use swe_edge_ingress_grpc_auth_bearer::{
    BearerInboundConfig, BearerInboundInterceptor, BearerSecret, AUTHORIZATION_HEADER,
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

fn mint(secret: &[u8], iss: &str, aud: &str, sub: &str, exp_offset_secs: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp = if exp_offset_secs >= 0 {
        now.saturating_add(exp_offset_secs as u64)
    } else {
        now.saturating_sub((-exp_offset_secs) as u64)
    };
    encode(
        &Header::new(Algorithm::HS256),
        &Claims { iss: iss.into(), aud: aud.into(), sub: sub.into(), iat: now, exp },
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

fn config(secret: &[u8]) -> BearerInboundConfig {
    BearerInboundConfig {
        secret: BearerSecret::Hs256 { secret: secret.to_vec() },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    }
}

fn req_with_bearer(value: &str) -> GrpcRequest {
    let mut headers = std::collections::HashMap::new();
    headers.insert(AUTHORIZATION_HEADER.to_string(), value.to_string());
    GrpcRequest::new("/svc/M", vec![], Duration::from_secs(1))
        .with_metadata(GrpcMetadata { headers })
}

/// @covers: valid token — subject republished under EXTRACTED_BEARER_SUBJECT.
#[test]
fn bearer_struct_inbound_valid_token_republishes_subject_int_test() {
    let secret = b"the-quick-brown-fox-jumps-over-32-bytes!";
    let token = mint(secret, "svc-a", "svc-b", "alice", 60);
    let interceptor = BearerInboundInterceptor::from_config(config(secret));
    let mut req = req_with_bearer(&format!("Bearer {token}"));
    interceptor.before_dispatch(&mut req).expect("valid token");
    assert_eq!(
        req.metadata.headers.get(EXTRACTED_BEARER_SUBJECT).map(String::as_str),
        Some("alice"),
    );
}

/// @covers: missing header returns Unauthenticated.
#[test]
fn bearer_struct_inbound_missing_header_returns_unauthenticated_int_test() {
    let interceptor = BearerInboundInterceptor::from_config(config(b"sec"));
    let mut req = GrpcRequest::new("/svc/M", vec![], Duration::from_secs(1));
    match interceptor.before_dispatch(&mut req) {
        Err(GrpcInboundError::Status(GrpcStatusCode::Unauthenticated, _)) => {}
        other => panic!("expected Unauthenticated, got {other:?}"),
    }
}

/// @covers: wrong secret returns Unauthenticated.
#[test]
fn bearer_struct_inbound_wrong_secret_returns_unauthenticated_int_test() {
    let token = mint(b"other-secret", "svc-a", "svc-b", "alice", 60);
    let interceptor = BearerInboundInterceptor::from_config(config(b"my-secret"));
    let mut req = req_with_bearer(&format!("Bearer {token}"));
    match interceptor.before_dispatch(&mut req) {
        Err(GrpcInboundError::Status(GrpcStatusCode::Unauthenticated, _)) => {}
        other => panic!("expected Unauthenticated, got {other:?}"),
    }
}

/// @covers: expired token returns Unauthenticated.
#[test]
fn bearer_struct_inbound_expired_token_returns_unauthenticated_int_test() {
    let secret = b"sec";
    let token = mint(secret, "svc-a", "svc-b", "alice", -30);
    let interceptor = BearerInboundInterceptor::from_config(config(secret));
    let mut req = req_with_bearer(&format!("Bearer {token}"));
    match interceptor.before_dispatch(&mut req) {
        Err(GrpcInboundError::Status(GrpcStatusCode::Unauthenticated, _)) => {}
        other => panic!("expected Unauthenticated, got {other:?}"),
    }
}

/// @covers: spoofed subject is stripped before re-injection.
#[test]
fn bearer_struct_inbound_strips_spoofed_subject_before_setting_verified_one_int_test() {
    let secret = b"sec";
    let token = mint(secret, "svc-a", "svc-b", "alice", 60);
    let interceptor = BearerInboundInterceptor::from_config(config(secret));
    let mut req = req_with_bearer(&format!("Bearer {token}"));
    req.metadata
        .headers
        .insert(EXTRACTED_BEARER_SUBJECT.to_string(), "spoofed-admin".into());
    interceptor.before_dispatch(&mut req).expect("valid token");
    assert_eq!(
        req.metadata.headers.get(EXTRACTED_BEARER_SUBJECT).map(String::as_str),
        Some("alice"),
        "verified subject must replace spoofed one",
    );
}
