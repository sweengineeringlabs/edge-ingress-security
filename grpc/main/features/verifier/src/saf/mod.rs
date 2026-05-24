//! Public facade — re-exports from `api/`.

mod verifier_svc;

pub use crate::api::bearer_interceptor::BearerTokenInterceptor;
pub use verifier_svc::create_config_builder;
