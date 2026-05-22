//! Builder for application configuration sourced from `config/application.toml`.

/// Builder for application configuration.
///
/// Counterpart for `config/application.toml`.
#[derive(Debug, Default)]
pub struct ApplicationConfigBuilder {
    _private: (),
}

impl ApplicationConfigBuilder {
    /// Construct a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consume the builder and return the finalised configuration.
    pub fn build(self) -> ApplicationConfigBuilder {
        self
    }
}
