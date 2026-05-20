//! Builder types for verifier application configuration.
//!
//! Impl blocks live in the `saf` layer. Struct shapes are declared here so
//! types are anchored in the interface layer per SEA rule 160.

/// Builder for application configuration.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ApplicationConfigBuilder {
    _private: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_builder_constructs() {
        let _b = ApplicationConfigBuilder::default();
    }
}
