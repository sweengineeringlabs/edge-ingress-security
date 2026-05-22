//! Fluent builder for [`ApplicationConfig`].

use crate::api::application_config::ApplicationConfig;

/// Fluent builder that constructs an [`ApplicationConfig`] from the
/// `[authz]` TOML section of `config/application.toml`.
#[derive(Debug, Default)]
pub struct ApplicationConfigBuilder {
    default_policy: Option<String>,
}

impl ApplicationConfigBuilder {
    /// Override the default authz policy (e.g. `"allow"` for open environments).
    pub fn with_default_policy(mut self, policy: impl Into<String>) -> Self {
        self.default_policy = Some(policy.into());
        self
    }

    /// Consume the builder and return the finished config.
    pub fn build(self) -> ApplicationConfig {
        ApplicationConfig {
            default_policy: self.default_policy.unwrap_or_else(|| "deny".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: build
    #[test]
    fn test_build_returns_deny_by_default() {
        let cfg = ApplicationConfigBuilder::default().build();
        assert_eq!(cfg.default_policy, "deny");
    }

    /// @covers: with_default_policy
    #[test]
    fn test_with_default_policy_overrides_default() {
        let cfg = ApplicationConfigBuilder::default()
            .with_default_policy("allow")
            .build();
        assert_eq!(cfg.default_policy, "allow");
    }
}
