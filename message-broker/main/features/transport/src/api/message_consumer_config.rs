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
        "message_consumer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: MessageConsumerConfig::default
    #[test]
    fn test_message_consumer_config_default_capacity_is_1024() {
        assert_eq!(MessageConsumerConfig::default().capacity, 1024);
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_message_consumer_config_section_name_matches_toml_key() {
        assert_eq!(
            <MessageConsumerConfig as swe_edge_configbuilder::ConfigSection>::section_name(),
            "message_consumer"
        );
    }
}
