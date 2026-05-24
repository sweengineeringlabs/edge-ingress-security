//! SAF service functions for the mTLS auth interceptor.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::traits::{Processor, Validator};

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Returns `true` when the mTLS interceptor is an authorization gate.
pub fn is_authorization_interceptor() -> bool {
    true
}

/// Returns `true` when the provided interceptor satisfies the [`Processor`]
/// contract — i.e., it is both `Send` and `Sync`.
pub fn is_processor<T: Processor>(_interceptor: &T) -> bool {
    true
}

/// Returns `true` when the provided value satisfies the [`Validator`]
/// contract.
pub fn is_validator<T: Validator>(_value: &T) -> bool {
    true
}
