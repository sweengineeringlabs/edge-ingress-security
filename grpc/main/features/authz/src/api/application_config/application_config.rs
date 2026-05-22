//! Authz application configuration — loaded from `config/application.toml`.

/// Runtime configuration for the authz feature.
#[derive(Debug, Clone)]
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
