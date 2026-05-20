//! Builder for application-level configuration loaded from `config/application.toml`.

/// Fluent builder that constructs application configuration from
/// `config/application.toml`.
///
/// Corresponds to the `[application]` section in the TOML file.
#[allow(dead_code)]
pub struct ApplicationConfigBuilder {
    name: String,
    version: String,
}

impl ApplicationConfigBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
        }
    }

    /// Set the application name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the application version string.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Return the configured application name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the configured application version.
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: name
    #[test]
    fn test_new_creates_builder_with_empty_name_and_default_version() {
        let b = ApplicationConfigBuilder::new();
        assert_eq!(b.name(), "");
        assert_eq!(b.version(), "0.1.0");
    }

    /// @covers: with_name
    #[test]
    fn test_with_name_sets_application_name() {
        let b = ApplicationConfigBuilder::new().with_name("my-app");
        assert_eq!(b.name(), "my-app");
    }

    /// @covers: with_version
    #[test]
    fn test_with_version_sets_application_version() {
        let b = ApplicationConfigBuilder::new().with_version("2.0.0");
        assert_eq!(b.version(), "2.0.0");
    }

    /// @covers: name
    #[test]
    fn test_name_returns_configured_application_name() {
        let b = ApplicationConfigBuilder::new().with_name("edge-http");
        assert_eq!(b.name(), "edge-http");
    }

    /// @covers: version
    #[test]
    fn test_version_returns_configured_application_version() {
        let b = ApplicationConfigBuilder::new().with_version("1.2.3");
        assert_eq!(b.version(), "1.2.3");
    }
}
