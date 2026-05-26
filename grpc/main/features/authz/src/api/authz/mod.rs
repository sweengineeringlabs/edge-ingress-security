//! Authz API — error, interceptor struct, and policy trait.

pub(crate) mod authz_policy;
pub(crate) mod impls;
pub(crate) mod interceptor;

pub use crate::api::types::authz::AuthzInterceptor;
pub use authz_policy::AuthzPolicy;
