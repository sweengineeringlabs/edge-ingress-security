//! Application-level configuration type.

/// Application-level configuration.
#[derive(Debug, Clone, Default)]
pub struct ApplicationConfig {
    /// Application name.
    pub name: String,
    /// Application version.
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_default_has_empty_fields() {
        let cfg = ApplicationConfig::default();
        assert_eq!(cfg.name, "");
        assert_eq!(cfg.version, "");
    }
}
