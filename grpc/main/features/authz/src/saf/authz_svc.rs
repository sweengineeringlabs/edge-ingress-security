//! SAF-level authz service functions.

use crate::api::application_config::ApplicationConfig;
use crate::api::traits::{Processor, Validator};

/// Returns `true` when the authz interceptor acts as an authorization gate.
pub fn is_authorization_interceptor() -> bool {
    true
}

/// Validates an [`ApplicationConfig`] and returns any configuration errors.
pub fn validate_application_config(cfg: &ApplicationConfig) -> Result<(), String> {
    cfg.validate()
}

/// Assert that a value satisfies the [`Processor`] contract.
///
/// This function is the SAF wrapper for the `Processor` marker trait —
/// it accepts any type that implements `Processor` and confirms its role.
/// Callers use this to verify that a concrete type participates in the
/// processing pipeline before wiring it into the server.
pub fn assert_is_processor<T: Processor>(_: &T) {}
