//! `swe_edge_ingress` — inbound gateway adapters.

// Public API items in a library crate are not "dead code" — they are
// consumed by downstream crates, not within this crate itself.
#![allow(dead_code)]
// MockFailureMode variants intentionally share the "Fail" prefix for
// clarity in test scenarios.
#![allow(clippy::enum_variant_names)]

mod api;
mod core;
mod provider;
mod saf;
mod state;

pub use api::application_config_builder::ApplicationConfigBuilder;
pub use api::architecture_config_builder::ArchitectureConfigBuilder;
pub use saf::*;
