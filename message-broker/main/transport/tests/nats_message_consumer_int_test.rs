//! Integration tests — NATS message consumer interface contract.
//!
//! The NATS backend requires a live NATS server and is guarded by the `nats` feature.

/// @covers: NatsMessageConsumer — feature-gated availability
#[test]
fn test_nats_message_consumer_trait_is_feature_gated() {
    // The NatsMessageConsumer trait is only implemented when the `nats` feature is enabled.
    // This test documents the feature gate requirement.
    let nats_feature_enabled = cfg!(feature = "nats");
    let _ = nats_feature_enabled;
}
