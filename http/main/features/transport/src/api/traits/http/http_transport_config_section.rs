//! `HttpTransportConfigSection` — marker trait for typed HTTP transport config sections.

/// Marker trait for HTTP transport configuration sections loadable via `swe-edge-configbuilder`.
pub trait HttpTransportConfigSection: swe_edge_configbuilder::ConfigSection {}

impl<T> HttpTransportConfigSection for T where T: swe_edge_configbuilder::ConfigSection {}
