//! API layer — policy trait, error type, identity adapter.

pub(crate) mod application;
pub(crate) mod authz;
pub mod error;
pub(crate) mod method;
pub mod traits;
pub mod types;

pub use error::AuthzError;
pub use types::{AuthzInterceptor, AuthzPolicy, MethodAclConfig, MethodAclPolicy};
