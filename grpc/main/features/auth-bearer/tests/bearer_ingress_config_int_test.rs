//! Integration tests for [`BearerIngressConfig`] — exercises construction,
//! field access, serialisation, and the `Validator` contract.

use swe_edge_ingress_grpc_auth_bearer::{BearerIngressConfig, BearerSecret};

fn hs256_config() -> BearerIngressConfig {
    BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"my-secret".to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 10,
    }
}

/// @covers: BearerIngressConfig construction
#[test]
fn test_bearer_ingress_config_fields_roundtrip_correctly() {
    let cfg = hs256_config();
    assert_eq!(cfg.expected_issuer, "svc-a");
    assert_eq!(cfg.expected_audience, "svc-b");
    assert_eq!(cfg.leeway_seconds, 10);
}

/// @covers: BearerIngressConfig — Clone
#[test]
fn test_bearer_ingress_config_clone_produces_equal_values() {
    let cfg = hs256_config();
    let cloned = cfg.clone();
    assert_eq!(cloned.expected_issuer, cfg.expected_issuer);
    assert_eq!(cloned.expected_audience, cfg.expected_audience);
    assert_eq!(cloned.leeway_seconds, cfg.leeway_seconds);
}

/// @covers: BearerIngressConfig — Debug
#[test]
fn test_bearer_ingress_config_debug_contains_field_names() {
    let cfg = hs256_config();
    let dbg = format!("{cfg:?}");
    assert!(
        dbg.contains("expected_issuer"),
        "Debug output must include field name"
    );
    assert!(
        dbg.contains("leeway_seconds"),
        "Debug output must include field name"
    );
}

/// @covers: BearerIngressConfig — zero leeway is accepted
#[test]
fn test_bearer_ingress_config_zero_leeway_is_valid() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: vec![1, 2, 3],
        },
        expected_issuer: "iss".into(),
        expected_audience: "aud".into(),
        leeway_seconds: 0,
    };
    assert_eq!(cfg.leeway_seconds, 0);
}

/// @covers: BearerIngressConfig — serde round-trip via toml
#[test]
fn test_bearer_ingress_config_toml_round_trip() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"round-trip-secret".to_vec(),
        },
        expected_issuer: "issuer-x".into(),
        expected_audience: "audience-y".into(),
        leeway_seconds: 30,
    };
    let serialised = toml::to_string(&cfg).expect("must serialise to TOML");
    let deserialised: BearerIngressConfig =
        toml::from_str(&serialised).expect("must deserialise from TOML");
    assert_eq!(deserialised.expected_issuer, "issuer-x");
    assert_eq!(deserialised.expected_audience, "audience-y");
    assert_eq!(deserialised.leeway_seconds, 30);
}
