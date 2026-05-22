//! SAF wrappers for the gRPC reflection service.

use crate::api::reflection::reflection_request::ReflectionRequest;
use crate::api::reflection::reflection_response::ReflectionResponse;
use crate::api::reflection::reflection_service::ReflectionService;
use crate::api::validator::Validator;

/// Process a single reflection request through the given service and return the response.
///
/// This is the primary SAF entry point for callers that hold a [`ReflectionService`]
/// and want to dispatch a decoded [`ReflectionRequest`].
pub fn handle_reflection(svc: &ReflectionService, req: ReflectionRequest) -> ReflectionResponse {
    svc.handle_request(req)
}

/// Validate a raw inbound frame using the built-in default validator.
///
/// Returns `Ok(())` for any well-formed payload. Use this wrapper in place of
/// importing the `Validator` trait directly.
pub fn validate_payload(raw: &[u8]) -> Result<(), String> {
    crate::core::validator::DefaultValidator.validate(raw)
}
