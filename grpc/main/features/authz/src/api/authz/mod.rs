//! Authz API — error, interceptor struct, and policy trait.

pub(crate) mod authz_error;
pub(crate) mod authz_interceptor;
pub(crate) mod authz_policy;

pub use authz_error::AuthzError;
pub use authz_interceptor::AuthzInterceptor;
pub use authz_policy::AuthzPolicy;
