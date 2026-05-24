//! API layer — config, error types, interceptor struct, and trait contracts.

pub mod error;
pub(crate) mod mtls;
pub(crate) mod mtls_auth_config;
pub(crate) mod mtls_auth_error;
pub(crate) mod mtls_auth_interceptor;
pub(crate) mod processor;
pub(crate) mod traits;
pub mod types;

pub use error::MtlsAuthError;
pub use mtls_auth_config::MtlsAuthConfig;
pub use mtls_auth_interceptor::MtlsAuthInterceptor;
