//! Application-level configuration type.

/// Application-level configuration.
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ApplicationConfig {
    /// Application name.
    pub name: String,
    /// Application version.
    pub version: String,
}

impl swe_edge_configbuilder::ConfigSection for ApplicationConfig {
    fn section_name() -> &'static str {
        const NAME: &str = "application";
        NAME
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_configbuilder::ConfigSection as _;

    #[test]
    fn test_application_config_default_has_empty_fields() {
        let cfg = ApplicationConfig::default();
        assert_eq!(cfg.name, "");
        assert_eq!(cfg.version, "");
    }

    /// @covers: section_name
    #[test]
    fn test_section_name_returns_application_key() {
        assert_eq!(ApplicationConfig::section_name(), "application");
    }
}
