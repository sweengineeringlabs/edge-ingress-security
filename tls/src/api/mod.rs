//! TLS API — error types and value objects.

pub(crate) mod error;
pub(crate) mod types;

pub(crate) use error::IngressTlsError;
pub(crate) use types::IngressTlsConfig;
pub(crate) mod acceptor;
pub(crate) mod traits;
