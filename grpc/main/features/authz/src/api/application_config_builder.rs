//! Builder for application-level configuration loaded from `config/application.toml`.

use std::path::PathBuf;

/// Fluent builder for application configuration loading.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationConfigBuilder {
    name: String,
    version: String,
    config_dirs: Vec<PathBuf>,
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
        }
    }
}

impl swe_edge_configbuilder::ConfigBuilder for ApplicationConfigBuilder {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
    }

    fn build_loader(self) -> impl swe_edge_configbuilder::Loader {
        let mut builder = swe_edge_configbuilder::create_config_builder()
            .with_name(self.name)
            .with_version(self.version);

        for dir in self.config_dirs {
            builder = builder.with_config_dir(dir);
        }

        builder.build_loader()
    }
}
