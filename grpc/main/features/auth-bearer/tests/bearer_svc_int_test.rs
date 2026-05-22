//! Integration tests for [`bearer_svc`] — exercises the public SAF service
//! functions exported from `saf/bearer_svc.rs`.

use swe_edge_ingress_grpc_auth_bearer::{
    extracted_bearer_subject_key, validate_bearer_config, BearerIngressConfig, BearerSecret,
};

/// @covers: extracted_bearer_subject_key — returns non-empty string
#[test]
fn test_extracted_bearer_subject_key_is_non_empty() {
    let key = extracted_bearer_subject_key();
    assert!(
        !key.is_empty(),
        "extracted_bearer_subject_key must not return an empty string"
    );
}

/// @covers: extracted_bearer_subject_key — has x-edge- prefix
#[test]
fn test_extracted_bearer_subject_key_has_x_edge_prefix() {
    let key = extracted_bearer_subject_key();
    assert!(
        key.starts_with("x-edge-"),
        "extracted_bearer_subject_key must begin with 'x-edge-', got: {key}"
    );
}

/// @covers: extracted_bearer_subject_key — is stable across calls
#[test]
fn test_extracted_bearer_subject_key_returns_same_value_on_repeated_calls() {
    let first = extracted_bearer_subject_key();
    let second = extracted_bearer_subject_key();
    assert_eq!(
        first, second,
        "extracted_bearer_subject_key must be stable across calls"
    );
}

/// @covers: validate_bearer_config — valid config returns Ok
#[test]
fn test_validate_bearer_config_valid_config_returns_ok() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"sec".to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    };
    assert!(
        validate_bearer_config(&cfg).is_ok(),
        "config with non-empty issuer and audience must validate successfully"
    );
}

/// @covers: validate_bearer_config — empty issuer returns Err
#[test]
fn test_validate_bearer_config_empty_issuer_returns_err() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"sec".to_vec(),
        },
        expected_issuer: "".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    };
    let result = validate_bearer_config(&cfg);
    assert!(result.is_err(), "empty issuer must fail validation");
    assert!(
        result.unwrap_err().contains("expected_issuer"),
        "error message must mention the invalid field"
    );
}

/// @covers: validate_bearer_config — empty audience returns Err
#[test]
fn test_validate_bearer_config_empty_audience_returns_err() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"sec".to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "".into(),
        leeway_seconds: 0,
    };
    let result = validate_bearer_config(&cfg);
    assert!(result.is_err(), "empty audience must fail validation");
    assert!(
        result.unwrap_err().contains("expected_audience"),
        "error message must mention the invalid field"
    );
}
