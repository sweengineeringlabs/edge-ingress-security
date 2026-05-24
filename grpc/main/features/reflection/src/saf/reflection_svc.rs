//! SAF wrappers for the gRPC reflection service.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::types::reflection_service::ReflectionService;
use crate::api::types::{ReflectionRequest, ReflectionResponse};

/// Creates a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Process a single reflection request through the given service and return the response.
///
/// This is the primary SAF entry point for callers that hold a [`ReflectionService`]
/// and want to dispatch a decoded [`ReflectionRequest`].
pub fn handle_reflection(svc: &ReflectionService, req: ReflectionRequest) -> ReflectionResponse {
    svc.handle_request(req)
}

/// Validate a raw inbound frame using the built-in default validator.
///
/// The default validator is permissive: it accepts any byte sequence, including
/// empty payloads. Returns `Ok(())` for every well-formed input.
pub fn validate_payload(_raw: &[u8]) -> Result<(), String> {
    Ok(())
}
