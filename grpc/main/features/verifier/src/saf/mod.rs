//! Public facade — re-exports from `api/`.

mod verifier_svc;

pub use crate::api::error::VerifierError;
pub use crate::api::types::BearerTokenInterceptor;
pub use verifier_svc::{create_config_builder, describe_processor, validate};
