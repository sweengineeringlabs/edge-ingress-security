//! SAF layer — public facade.

mod authz_svc;

pub use crate::api::application::ApplicationConfig;
pub use crate::api::{AuthzError, AuthzInterceptor, AuthzPolicy, MethodAclConfig, MethodAclPolicy};
pub use authz_svc::{
    assert_is_processor, create_config_builder, is_authorization_interceptor,
    validate_application_config,
};
