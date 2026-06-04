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
