//! API layer — config, error types, interceptor struct, and trait contracts.

pub(crate) mod application_config_builder;
pub(crate) mod mtls_auth_config;
pub(crate) mod mtls_auth_error;
pub(crate) mod mtls_auth_interceptor;
pub(crate) mod processor;
pub(crate) mod traits;

pub use mtls_auth_config::MtlsAuthConfig;
pub use mtls_auth_error::MtlsAuthError;
pub use mtls_auth_interceptor::MtlsAuthInterceptor;
