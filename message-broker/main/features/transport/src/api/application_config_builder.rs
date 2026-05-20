//! Application configuration builder for the ingress message-broker transport crate.

/// Builds runtime configuration for the ingress message consumer from
/// `config/application.toml` settings.
#[allow(dead_code)]
pub struct ApplicationConfigBuilder {
    /// Maximum in-memory channel capacity (default: 1024).
    pub capacity: usize,
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}

impl ApplicationConfigBuilder {
    /// Construct with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the channel capacity.
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_builder_new_returns_defaults() {
        let cfg = ApplicationConfigBuilder::new();
        assert_eq!(cfg.capacity, 1024);
    }

    /// @covers: with_capacity
    #[test]
    fn test_application_config_builder_with_capacity_overrides_default() {
        let cfg = ApplicationConfigBuilder::new().with_capacity(256);
        assert_eq!(cfg.capacity, 256);
    }
}
