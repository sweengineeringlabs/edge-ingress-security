//! Integration tests for the `Validator` trait (Rule 105).
//!
//! Exercises the `validate` SAF function and the `Validator` implementation
//! on `HttpConfig` end-to-end.

use swe_edge_ingress_http::{validate, HttpConfig};

/// @covers: validate
#[test]
fn test_validate_returns_ok_for_default_http_config() {
    let cfg = HttpConfig::default();
    assert!(
        validate(&cfg).is_ok(),
        "expected Ok for default HttpConfig, got: {:?}",
        validate(&cfg)
    );
}

/// @covers: validate
#[test]
fn test_validate_returns_err_when_timeout_secs_is_zero() {
    let cfg = HttpConfig {
        timeout_secs: 0,
        ..Default::default()
    };
    let result = validate(&cfg);
    assert!(result.is_err(), "expected Err for zero timeout_secs");
    let msg = result.unwrap_err();
    assert!(
        msg.contains("timeout_secs"),
        "expected error to mention 'timeout_secs', got: {msg}"
    );
}

/// @covers: validate
#[test]
fn test_validate_returns_err_when_connect_timeout_secs_is_zero() {
    let cfg = HttpConfig {
        connect_timeout_secs: 0,
        ..Default::default()
    };
    let result = validate(&cfg);
    assert!(
        result.is_err(),
        "expected Err for zero connect_timeout_secs"
    );
    let msg = result.unwrap_err();
    assert!(
        msg.contains("connect_timeout_secs"),
        "expected error to mention 'connect_timeout_secs', got: {msg}"
    );
}

/// @covers: validate
#[test]
fn test_validate_returns_ok_for_custom_valid_config() {
    let cfg = HttpConfig::with_base_url("https://api.example.com").with_timeout(60);
    assert!(
        validate(&cfg).is_ok(),
        "expected Ok for custom valid config, got: {:?}",
        validate(&cfg)
    );
}
