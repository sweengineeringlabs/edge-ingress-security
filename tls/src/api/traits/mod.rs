//! TLS API traits.

pub mod tls_extension;

pub use tls_extension::TlsExtension;
pub mod validator;
pub use validator::Validator;
