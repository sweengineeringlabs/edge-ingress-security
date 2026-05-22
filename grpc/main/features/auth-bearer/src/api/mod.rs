//! API layer — config, error, and constants for the inbound bearer interceptor.

pub(crate) mod bearer_auth_config;
pub(crate) mod bearer_auth_error;
pub(crate) mod bearer_ingress_interceptor;
pub(crate) mod jwt_claims;
pub(crate) mod metadata_keys;

pub use bearer_auth_config::{BearerIngressConfig, BearerSecret};
pub use bearer_auth_error::BearerAuthError;
pub use bearer_ingress_interceptor::BearerIngressInterceptor;
pub use metadata_keys::{AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT};
