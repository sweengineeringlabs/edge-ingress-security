//! API layer — config, error types, interceptor struct, and trait contracts.

pub mod error;
pub(crate) mod mtls;
pub mod traits;
pub mod types;

pub use error::MtlsAuthError;
pub use mtls::{MtlsAuthConfig, MtlsAuthInterceptor};
