//! Integration tests for [`BearerIngressInterceptor`] API surface —
//! verifies construction via `from_config` and that the type's public
//! interface is complete.

use swe_edge_ingress_grpc_auth_bearer::{
    BearerIngressConfig, BearerIngressInterceptor, BearerSecret,
};

fn rs256_config() -> BearerIngressConfig {
    BearerIngressConfig {
        secret: BearerSecret::Rs256 { public_pem: vec![] },
        expected_issuer: "issuer".into(),
        expected_audience: "audience".into(),
        leeway_seconds: 0,
    }
}

/// @covers: BearerIngressInterceptor::from_config
#[test]
fn test_from_config_hs256_produces_interceptor() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"super-secret-key-at-least-32-bytes!".to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    };
    let _interceptor = BearerIngressInterceptor::from_config(cfg);
    // Reaching here means from_config does not panic and returns a valid value.
}

/// @covers: BearerIngressInterceptor::from_config — Rs256 variant
#[test]
fn test_from_config_rs256_produces_interceptor() {
    let _interceptor = BearerIngressInterceptor::from_config(rs256_config());
}

/// @covers: BearerIngressInterceptor — is Send
#[test]
fn test_bearer_ingress_interceptor_is_send() {
    fn require_send<T: Send>() {}
    require_send::<BearerIngressInterceptor>();
}

/// @covers: BearerIngressInterceptor — is Sync
#[test]
fn test_bearer_ingress_interceptor_is_sync() {
    fn require_sync<T: Sync>() {}
    require_sync::<BearerIngressInterceptor>();
}
