//! SAF layer — public facade.

mod mtls_svc;

pub use crate::api::{MtlsAuthConfig, MtlsAuthError, MtlsAuthInterceptor};
pub use swe_edge_configbuilder::create_config_builder;
pub use mtls_svc::{is_authorization_interceptor, is_processor, is_validator};
