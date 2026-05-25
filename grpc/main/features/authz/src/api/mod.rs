//! API layer — policy trait, error type, identity adapter.

pub(crate) mod application;
pub(crate) mod authz;
pub mod error;
pub(crate) mod method;
pub(crate) mod processor;
pub(crate) mod traits;
pub mod types;

pub use error::AuthzError;
pub use types::{
    ApplicationConfig, ApplicationConfigBuilder, AuthzInterceptor, AuthzPolicy, MethodAclConfig,
    MethodAclPolicy,
};
