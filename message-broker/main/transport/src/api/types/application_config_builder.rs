//! `ApplicationConfigBuilder` — builder for `config/application.toml` settings.

/// Builder for the message consumer application configuration.
///
/// Maps to the `[message_consumer]` section of `config/application.toml`.
#[derive(Debug, Clone)]
pub struct ApplicationConfigBuilder {
    /// Maximum in-memory channel capacity.
    pub capacity: usize,
}

impl ApplicationConfigBuilder {
    /// Create a new builder with default capacity.
    pub fn new() -> Self {
        Self { capacity: 1024 }
    }

    /// Override the channel capacity.
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
