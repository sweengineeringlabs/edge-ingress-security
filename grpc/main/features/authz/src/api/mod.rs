//! API layer — policy trait, error type, identity adapter.

pub(crate) mod application_config;
pub(crate) mod authz;
pub(crate) mod authz_interceptor;
pub(crate) mod method;
pub(crate) mod method_acl_policy;
pub(crate) mod processor;
pub(crate) mod traits;

pub use authz::AuthzError;
pub use authz::AuthzInterceptor;
pub use authz::AuthzPolicy;
pub use method::MethodAclConfig;
pub use method::MethodAclPolicy;
