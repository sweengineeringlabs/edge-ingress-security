//! Builder for application configuration loaded from `config/application.toml`.

use super::application_config::ApplicationConfig;

/// Builder for application-level configuration loaded from `config/application.toml`.
///
/// Provides a fluent interface for constructing the application configuration
/// used at runtime initialisation.
#[derive(Debug, Clone, Default)]
pub struct ApplicationConfigBuilder {
    name: Option<String>,
    version: Option<String>,
}

impl ApplicationConfigBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the application version string.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Build and return the application configuration.
    ///
    /// `name` and `version` default to empty strings when not set.
    pub fn build(self) -> ApplicationConfig {
        ApplicationConfig {
            name: self.name.unwrap_or_default(),
            version: self.version.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_builder() {
        let b = ApplicationConfigBuilder::new();
        assert!(b.name.is_none());
        assert!(b.version.is_none());
    }

    /// @covers: name
    #[test]
    fn test_name_sets_application_name() {
        let cfg = ApplicationConfigBuilder::new().name("my-app").build();
        assert_eq!(cfg.name, "my-app");
    }

    /// @covers: version
    #[test]
    fn test_version_sets_application_version() {
        let cfg = ApplicationConfigBuilder::new().version("1.2.3").build();
        assert_eq!(cfg.version, "1.2.3");
    }

    /// @covers: build
    #[test]
    fn test_build_returns_config_with_defaults_for_unset_fields() {
        let cfg = ApplicationConfigBuilder::new().build();
        assert_eq!(cfg.name, "");
        assert_eq!(cfg.version, "");
    }
}
