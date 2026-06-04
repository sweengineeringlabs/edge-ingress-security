//! Authz application configuration — loaded from `config/application.toml`.

/// Runtime configuration for the authz feature.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct ApplicationConfig {
    /// The default authz policy to apply (`"deny"` or `"allow"`).
    pub default_policy: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            default_policy: "deny".into(),
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for ApplicationConfig {
    fn section_name() -> &'static str {
        const NAME: &str = "authz";
        NAME
    }
}
