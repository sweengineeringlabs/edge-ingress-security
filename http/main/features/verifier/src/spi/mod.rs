//! Service-provider extension hooks for HTTP verifier configuration.

/// Marker trait for typed HTTP verifier configuration sections.
#[allow(dead_code)]
pub(crate) trait HttpVerifierConfigSection: swe_edge_configbuilder::ConfigSection {}

impl<T> HttpVerifierConfigSection for T where T: swe_edge_configbuilder::ConfigSection {}
