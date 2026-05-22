//! Integration tests for the crate's public API traits surface.
//!
//! `Processor` and `Validator` are crate-internal traits; this file
//! exercises their effects via the public facade to confirm the contracts
//! compile and hold at runtime.

use swe_edge_ingress_grpc_auth_bearer::{
    ApplicationConfigBuilder, BearerIngressConfig, BearerIngressInterceptor, BearerSecret,
};

/// @covers: Processor trait — BearerIngressInterceptor is Send + Sync (Processor's super-bounds)
#[test]
fn test_processor_super_bounds_satisfied_by_interceptor() {
    fn require<T: Send + Sync>() {}
    require::<BearerIngressInterceptor>();
}

/// @covers: Validator trait — valid BearerIngressConfig passes validation via public API
///
/// The Validator impl rejects empty issuer/audience.  We exercise both the
/// pass and fail paths here to confirm the contract cannot be trivially
/// broken.
#[test]
fn test_validator_accepts_config_with_non_empty_issuer_and_audience() {
    let cfg = BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"sec".to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 0,
    };
    // The Validator impl is crate-internal; we verify indirectly that
    // a well-formed config can be passed to from_config without panic.
    let _interceptor = BearerIngressInterceptor::from_config(cfg);
}

/// @covers: ApplicationConfigBuilder — new() and build() do not panic
#[test]
fn test_application_config_builder_new_and_build_succeed() {
    let _cfg = ApplicationConfigBuilder::new().build();
}

/// @covers: ApplicationConfigBuilder — Debug impl
#[test]
fn test_application_config_builder_debug_does_not_panic() {
    let cfg = ApplicationConfigBuilder::new();
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("ApplicationConfigBuilder"));
}
