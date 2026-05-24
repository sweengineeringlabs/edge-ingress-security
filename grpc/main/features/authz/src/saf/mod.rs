//! SAF layer — public facade.

mod authz_svc;

pub use crate::api::application_config::ApplicationConfig;
pub use crate::api::{AuthzError, AuthzInterceptor, AuthzPolicy, MethodAclConfig, MethodAclPolicy};
pub use swe_edge_configbuilder::create_config_builder;
pub use authz_svc::{
    assert_is_processor, is_authorization_interceptor, validate_application_config,
};
