//! API layer — policy trait, error type, identity adapter.

pub(crate) mod application_config;
pub(crate) mod authz;
pub(crate) mod authz_interceptor;
pub mod error;
pub(crate) mod method;
pub(crate) mod method_acl_policy;
pub(crate) mod processor;
pub(crate) mod traits;
pub mod types;

pub use error::AuthzError;
pub use types::{
    ApplicationConfig, AuthzInterceptor, AuthzPolicy, MethodAclConfig, MethodAclPolicy,
};
