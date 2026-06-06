//! Typed configuration for the ingress message consumer.

/// Runtime configuration for the ingress message consumer.
///
/// Loaded from the `[message_consumer]` section of `application.toml`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MessageConsumerConfig {
    /// Maximum in-memory channel capacity.
    pub capacity: usize,
}

impl Default for MessageConsumerConfig {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}

impl swe_edge_configbuilder::ConfigSection for MessageConsumerConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "message_consumer"
    }
}
