//! ApplicationConfigBuilder type for inbound adapter configuration.

use std::sync::Arc;

use crate::api::inbound_source::InboundSource;

/// ApplicationConfigBuilder for inbound adapter configuration.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ApplicationConfigBuilder;

impl ApplicationConfigBuilder {
    /// Construct with default configuration.
    pub fn new() -> Self {
        Self
    }
}

/// Build the default local-filesystem inbound source.
pub fn build_file_input() -> Arc<dyn InboundSource> {
    Arc::new(crate::core::file::LocalFileSource)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new_returns_default() {
        let _ = ApplicationConfigBuilder::new();
    }

    /// @covers: build_file_input
    #[test]
    fn test_build_file_input_returns_arc_inbound_source() {
        let src = build_file_input();
        let _ = src.file_exists(std::path::Path::new(".")).unwrap();
    }
}
