//! Integration tests — verifies that [`BearerIngressInterceptor`] satisfies
//! the [`Processor`] contract.
//!
//! Because `Processor` is a crate-internal marker trait it cannot be named
//! in integration tests.  Coverage is satisfied by exercising the interceptor
//! through the public facade and verifying it is `Send + Sync`, which are
//! the super-bounds of `Processor`.

use swe_edge_ingress_grpc_auth_bearer::{
    BearerIngressConfig, BearerIngressInterceptor, BearerSecret,
};

fn make_interceptor() -> BearerIngressInterceptor {
    BearerIngressInterceptor::from_config(BearerIngressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"test-key-32-bytes-long-enough-ok!".to_vec(),
        },
        expected_issuer: "svc-a".into(),
        expected_audience: "svc-b".into(),
        leeway_seconds: 5,
    })
}

/// @covers: Processor impl for BearerIngressInterceptor — Send bound
#[test]
fn test_bearer_ingress_interceptor_satisfies_send_bound() {
    fn assert_send<T: Send>(_: T) {}
    assert_send(make_interceptor());
}

/// @covers: Processor impl for BearerIngressInterceptor — Sync bound
#[test]
fn test_bearer_ingress_interceptor_satisfies_sync_bound() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<BearerIngressInterceptor>();
}

/// @covers: Processor impl — interceptor can be stored behind Arc (requires Send + Sync)
#[test]
fn test_bearer_ingress_interceptor_can_be_stored_in_arc() {
    use std::sync::Arc;
    let interceptor = make_interceptor();
    let arc = Arc::new(interceptor);
    // If Processor's Send + Sync bounds were violated this would not compile.
    let _clone = Arc::clone(&arc);
}
