//! Service-provider extension hooks for HTTP transport configuration.

/// Marker trait for typed HTTP transport configuration sections.
#[allow(dead_code)]
pub trait HttpTransportConfigSection: swe_edge_configbuilder::ConfigSection {}

impl<T> HttpTransportConfigSection for T where T: swe_edge_configbuilder::ConfigSection {}
